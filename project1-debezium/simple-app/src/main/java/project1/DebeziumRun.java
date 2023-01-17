package project1;

import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.consumer.ConsumerRecords;
import org.apache.kafka.clients.consumer.KafkaConsumer;
import org.json.JSONObject;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.time.temporal.ChronoUnit;
import java.util.List;
import java.util.Properties;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public class DebeziumRun {

    private static final ExecutorService executor = Executors.newFixedThreadPool(2);
    private static final Logger log = LoggerFactory.getLogger("DebeziumRun");

    public static void main(String[] args) {
        // Configure + launch kafka consumer
        executor.execute(DebeziumRun::kafkaConsumer);
    }

    private static void kafkaConsumer() {

        // Generic consumer configuration. No debezium specific info here.
        var config = new Properties();
        config.put(ConsumerConfig.CLIENT_ID_CONFIG, "client-1");
        config.put(ConsumerConfig.GROUP_ID_CONFIG, "inventory-consumer");
        config.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG, "localhost:9092");
        config.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, "org.apache.kafka.common.serialization.StringDeserializer");
        config.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, "org.apache.kafka.common.serialization.StringDeserializer");
        var consumer = new KafkaConsumer<String, String>(config);

        // Depending on configuration, topic names could either be generated automatically, have or not have a prefix (dbserver in this case)
        // or manually.
        consumer.subscribe(List.of("dbserver1.inventory.customers"));
        while (true) {
            var records = consumer.poll(Duration.of(10, ChronoUnit.SECONDS));
            try {
                process(records); // application-specific processing
                consumer.commitSync();
            } catch (RuntimeException ex) {
                log.error("encountered exception while processing records", ex);
            }
        }
    }

    private static void process(ConsumerRecords<String, String> kafkaRecords) {
        System.out.println("received '" + kafkaRecords.count() + "' records");
        for (var kafkaRecord : kafkaRecords) {

            System.out.println("received a record partition: '" + kafkaRecord.partition() + "' offset: '" + kafkaRecord.offset() + "'");

            // Debezium will send a Kafka Message with ID, but without value. This only happens if a record is deleted
            // from the database. When a record is deleted you get two events: first with "after" == null (because it was
            // deleted) + this "special" message without value. The messages without value serve as tombstones for
            // compacted topic. This example uses automatic topic generation, so all topics are compacted.
            // For more info on compacted topics read here: https://kafka.apache.org/documentation/#compaction
            if (kafkaRecord.value() == null) {
                System.out.println("received a null record. Skipping");
                continue;
            }

            var recordAsJson = new JSONObject(kafkaRecord.value());

            // By default, a value have two json objects: "schema" which serve as json schema (https://json-schema.org) and "payload".
            // The payload contains a lot of different information such as what's the source of message, what is the operation
            // and most importantly the "before" and "after" records for a row.
            var payload = recordAsJson.getJSONObject("payload");
            var operation = payload.getString("op");
            var operationAsString = switch (operation) {
                case "u" -> "UPDATE";
                case "r" -> "READ";
                case "d" -> "DELETE";
                case "c" -> "CREATE";
                default -> new RuntimeException("unhandled operation '" + operation + "'");
            };
            System.out.println("Operation: '" + operationAsString + "' ");

            var before = payload.get("before");
            var after = payload.get("after");

            // For "c" — CREATE records "before" will be empty, because they were just created.
            if (before == null) {
                System.out.println("object before: not present");
            } else {
                System.out.println("object before: " + before);
            }

            // For "d" — DELETE records "after" will be empty, because they were just deleted.
            if (after == null) {
                System.out.println("object after: not present");
            } else {
                System.out.println("object after: " + after);
            }
        }
    }
}
