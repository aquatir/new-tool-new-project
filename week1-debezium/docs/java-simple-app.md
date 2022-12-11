# Java simple app

Part 1: Running the same config as in [intro.md](intro.md) only in docker-compose now + applying a listener.

Part 2: Running a Java app that will publish and read debezium events.

## Running and configuring a debezium connector.

Navigate to `simple-app` project to view the simple app.

Run docker-compose to run postgres, kafka and debezium connector

```shell
docker compose -f docker-compose-postgres.yaml up
```

Add a connector. Notice the hostname is `postgres:5432`, because the connector runs inside docker network. Change
appropriately.

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

Connect to postgres and execute inserts/update/deletes. You'd see the output in the topic you are listening too.

```shell
psql -h localhost -p 5432 -U postgres
```

You're beautiful :>

## Connecting to database with an app

