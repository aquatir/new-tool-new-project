# Java simple app

Part 1: Running the same config as in [intro.md](../docs/intro.md) only in docker-compose now + applying a listener.

Part 2: Running a Java app that will read Debezium events.

## Running and configuring a debezium connector.

Navigate to `simple-app` project to view the simple app.

Run docker-compose to run postgres, kafka and debezium connector

```shell
docker compose -f docker-compose-postgres.yaml up
```

Add a connector. Notice the hostname is `postgres:5432`, because the connector runs inside docker network. Change
appropriately. This configuration will read all tables in schema `inventory` and create topics for them using a prefix 
`dbserver1`. You'll end up with a bunch of topics such as
- `dbserver1.inventory.customers`
- `dbserver1.inventory.orders`
- etc.

```shell
curl -i -X POST -H "Accept:application/json" -H "Content-Type:application/json" localhost:8083/connectors/ -d '{"name": "inventory-connector","config": {"connector.class": "io.debezium.connector.postgresql.PostgresConnector","tasks.max": "1","database.hostname": "postgres","database.port": "5432","database.user": "postgres","database.password": "postgres","database.dbname" : "postgres","database.server.name": "dbserver1","schema.include.list": "inventory", "topic.prefix": "dbserver1"}}'
```

Check that added successfully:

```shell
curl -H "Accept:application/json" localhost:8083/connectors/
```

Check that new topics were created. I use local `kcat` here, but you should also be able to
use `quay.io/debezium/kafka:2.0 watch-topic` image.

```shell
kcat -b localhost:9092 -L
```

Connect to one of the topics

```shell
kcat -b localhost:9092 -t dbserver1.inventory.customers
```

You should see a bunch of rows that debezium read from the database. 

You're beautiful :>

## Launch the app and generate new events

Launch the app in `simple-app` directory either via IDE, or with gradle. NOTE: you must be in `week1-debezium` directory because only it has `gradlew` script. 
```shell
./gradlew :simple-app:run
```

Connect to database 

```shell
psql -h localhost -p 5432 -U postgres
```

Execute a bunch of SQL commands
```sql
insert into inventory.customers(first_name, last_name, email)
VALUES ('aza', 'zafa', 'asfasdfdfcadfdfsfdc@gmail.com');

update inventory.customers
    set first_name = 'kekw'
    where id = 1005;

delete from inventory.customers
    where id = 1005;
```

See the app's output. You'll see 4 events generated. First 3 are "CREATE", "UPDATE" and "DELETE" events. The last on is 
a special tombstone event. This event doesn't have a `value`. Refer the codebase to see the setup and much more explanations! 
