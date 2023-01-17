package project4;

import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.CqlSessionBuilder;

import java.net.InetSocketAddress;
import java.util.UUID;
import java.util.stream.Stream;

public class CassandraRun {

    public static void main(String[] args) {

        System.out.println("****");
        System.out.println("Starting Cassandra example");
        System.out.println("****");
        System.out.println();

        try (var session = newSession().build()) {

            // return current version
            var rs1 = session.execute("select release_version from system.local");
            System.out.println(rs1.one().getString("release_version"));

            // delete console data if present
            session.execute("delete from first_keyspace.customers where id in (af7e23d8-742c-4b24-9b6a-1492e8a570c8, 35430e28-e0a5-41cb-aac5-94275f2d9a1f)");

            // insert data again
            session.execute("""
                INSERT INTO first_keyspace.customers(id, first_name, second_name)
                values (af7e23d8-742c-4b24-9b6a-1492e8a570c8, 'Ivan', 'Ivanov')
                """);
            session.execute("""
                INSERT INTO first_keyspace.customers(id, first_name, second_name) 
                values (35430e28-e0a5-41cb-aac5-94275f2d9a1f, 'Petr', 'Petrov')
                """);

            // return all data from a row in DB
            var rs2 = session.execute("select * from first_keyspace.customers");

            System.out.println("rows of first_keyspace.customers");
            for (var row : rs2.all()) {
                System.out.printf("id: '%s', first_name: '%s', last_name: '%s'%n", row.getUuid("id"),
                        row.getString("first_name"), row.getString("second_name"));
            }

            var uuids = Stream.generate(UUID::randomUUID)
                    .limit(1_000)
                    .toList();
            for (var uuid : uuids) {
                session.execute("INSERT INTO first_keyspace.customers(id, first_name, second_name) values (?, 'Ivan', 'Ivanov')", uuid);
            }

            for (var uuid : uuids) {
                session.execute("delete from first_keyspace.customers where id = ?", uuid);
            }
        }
    }

    private static CqlSessionBuilder newSession() {
        // if you don't provide contact points and local datcentre1 (e.g. CqlSession.builder().builder(), it will still
        // work, because this is a default config
        return CqlSession.builder()
                .addContactPoint(new InetSocketAddress("localhost", 9042))
                .withLocalDatacenter("datacenter1")
                ;
    }
}
