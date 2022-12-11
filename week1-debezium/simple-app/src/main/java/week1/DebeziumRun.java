package week1;

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
        executor.execute(DebeziumRun::kafkaConsumer);
    }

    private static void kafkaConsumer() {
        var config = new Properties();
        config.put(ConsumerConfig.CLIENT_ID_CONFIG, "client-1");
        config.put(ConsumerConfig.GROUP_ID_CONFIG, "inventory-consumer");
        config.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG, "localhost:9092");
        config.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, "org.apache.kafka.common.serialization.StringDeserializer");
        config.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, "org.apache.kafka.common.serialization.StringDeserializer");
        var consumer = new KafkaConsumer<String, String>(config);

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
            if (kafkaRecord == null) {
                System.out.println("received a null record. Skipping");
                continue;
            }
            System.out.println("received a record partition: '" + kafkaRecord.partition() + "' offset: '" + kafkaRecord.offset() + "'");
            if (kafkaRecord.value() == null) {
                System.out.println("received a record without value. Skipping");
                continue;
            }
            var recordAsJson = new JSONObject(kafkaRecord.value());
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

            if (before == null) {
                System.out.println("object before: not present");
            } else {
                System.out.println("object before: " + before);
            }

            if (after == null) {
                System.out.println("object after: not present");
            } else {
                System.out.println("object after: " + after);
            }
        }
    }
}
