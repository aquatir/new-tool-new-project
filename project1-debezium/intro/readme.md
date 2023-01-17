# Debezium intro

This intro
follow [official debezium getting started tutorial](https://debezium.io/documentation/reference/stable/tutorial.html).

## Starting everything up

We will start debezium using a bunch of consoles and `docker`. Debezium uses Kafka, so first we'll have to start it.
Then we'll continue to starting database — MqSQL in this case, a shell for it and finally debezium Kafka Connectors.

When everything is up and running, we'll connect to one of Debezium's topic and watch it for events. We'd delete,
update, and create a bunch of row to see what types of events are generated. Finally, we'd stop debezium connector, make
a few changes to data, start it again and make sure that new events are generated and passed further.

### Start `Zookeeper`

```shell
docker run -it --rm --name zookeeper -p 2181:2181 -p 2888:2888 -p 3888:3888 quay.io/debezium/zookeeper:2.0
```

### Start `Kafka`

```shell
docker run -it --rm --name kafka -p 9092:9092 --link zookeeper:zookeeper quay.io/debezium/kafka:2.0
```

### Start `MySQL`.

`MySQL` in this example is pre-configured with `inventory` database ready that you'll use as a part of tutorial.

#### Start DB

```shell
docker run -it --rm --name mysql -p 3306:3306 -e MYSQL_ROOT_PASSWORD=debezium -e MYSQL_USER=mysqluser -e MYSQL_PASSWORD=mysqlpw quay.io/debezium/example-mysql:2.0
```

#### Login into `MySQL` console

```shell
docker run -it --rm --name mysqlterm --link mysql --rm mysql:8.0 sh -c 'exec mysql -h"$MYSQL_PORT_3306_TCP_ADDR" -P"$MYSQL_PORT_3306_TCP_PORT" -uroot -p"$MYSQL_ENV_MYSQL_ROOT_PASSWORD"'
```

#### Verify data is present

```
use inventory;
show tables;
SELECT * FROM customers;
```

### Start Kafka Connect

#### Start container

```shell
docker run -it --rm --name connect -p 8083:8083 -e GROUP_ID=1 -e CONFIG_STORAGE_TOPIC=my_connect_configs -e OFFSET_STORAGE_TOPIC=my_connect_offsets -e STATUS_STORAGE_TOPIC=my_connect_statuses --link kafka:kafka --link mysql:mysql quay.io/debezium/connect:2.0
```

#### Verify that it works with REST API

Kafka connect exposes REST API, so you can verify that it works

```shell
curl -H "Accept:application/json" localhost:8083/
curl -H "Accept:application/json" localhost:8083/connectors/
```

The last `curl` shows that there are zero connectors established just yet.

#### Register a connector

The connector will monitor DB's `binlog`. You can use `http` API to register a connector

```shell
curl -i -X POST -H "Accept:application/json" -H "Content-Type:application/json" localhost:8083/connectors/ -d '{ "name": "inventory-connector", "config": { "connector.class": "io.debezium.connector.mysql.MySqlConnector", "tasks.max": "1", "database.hostname": "mysql", "database.port": "3306", "database.user": "debezium", "database.password": "dbz", "database.server.id": "184054", "topic.prefix": "dbserver1", "database.include.list": "inventory", "schema.history.internal.kafka.bootstrap.servers": "kafka:9092", "schema.history.internal.kafka.topic": "schemahistory.inventory" } }'
```

now check that the connector was in fact registered

```shell
curl -H "Accept:application/json" localhost:8083/connectors/
```

Also check the definition of connector

```shell
curl -i -X GET -H "Accept:application/json" localhost:8083/connectors/inventory-connector
```

## Exploring the result

If you look at the output of `connect` container you'd see that is got through "snapshot" process and created a bunch of
topics for your data with a prefix `dbserver1`. This prefix was specified in the connect job posted with http API
before.

- `dbserver1`
- `dbserver1.inventory.products`
- `dbserver1.inventory.products_on_hand`
- `dbserver1.inventory.customers`
- `dbserver1.inventory.orders`

### Start a watch topic

Now lets start a watch topic and see what events are generated for `inventory.customers` topic.

```shell
docker run -it --rm --name watcher --link zookeeper:zookeeper --link kafka:kafka quay.io/debezium/kafka:2.0 watch-topic -a -k dbserver1.inventory.customers
```

You'll see 4 events, each consisting of 2 json objects. The first one is schema+payload of a key and then the
schema+payload of data itself. Note that schema also include debezium specific fields, not just your database schema.

Now try to change a row

```sql
UPDATE customers
SET first_name='Anne Marie'
WHERE id = 1004;
```

and see that new "update" event with `"op": "u"` is generated.

Now try to delete a row

```sql
DELETE
FROM addresses
WHERE customer_id = 1004;
DELETE
FROM customers
WHERE id = 1004;

```

Now you'll see two events:

- A "delete" event `"op": "d"`.
- An empty event with `{"schema": null,"payload": null}`. It may also be viewed as simply `null` in console.

The purpose of this event is to support log-compacted topics in Kafka. Specifically this event will delete an entry from
log-compacted topic allowing Kafka to discard all events associated with this entry.

## Stopping connect client

Let's see what happen when you stop connect client. Execute

```shell
docker stop connect
```

And execute a couple of SQL commands

```sql
INSERT INTO customers
VALUES (default, "Sarah", "Thompson", "kitt@acme.com");
INSERT INTO customers
VALUES (default, "Kenneth", "Anderson", "kander@acme.com");
```

You can see that no new events are being generated in watch topic because debezium kafka connect is turned off.

Now let's start connect container again

```shell
docker run -it --rm --name connect -p 8083:8083 -e GROUP_ID=1 -e CONFIG_STORAGE_TOPIC=my_connect_configs -e OFFSET_STORAGE_TOPIC=my_connect_offsets -e STATUS_STORAGE_TOPIC=my_connect_statuses --link zookeeper:zookeeper --link kafka:kafka --link mysql:mysql quay.io/debezium/connect:2.0
```

Switch to watch topic. After a few seconds, you should see two new events with type "create" `"op": "c"` generated.

The tutorial is over. You can stop all container with

```shell
docker stop mysqlterm watcher connect mysql kafka zookeeper
```

Make sure that all containers are now stopped by executing

```shell
docker ps
```
