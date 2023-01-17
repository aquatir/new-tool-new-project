# Intro

## 

Running Cassandra and connecting via Java and console

Navigate to `docker` directory and run
```shell
docker-compose up
```

You can check out Cassandra's default config 
```shell
docker exec -it cass_cluster /bin/bash
cd /etc/cassandra
cat cassandra.yaml
```

There are a lot of different configs apart from `cassandra.yaml`. 

## Using console


After you start Cassandra, wait until this line apper in logs
```
Starting listening for CQL clients on /0.0.0.0:9042 (unencrypted).
```

And then run `cqlsh` console
```shell
docker exec -it cass_cluster cqlsh
```

From there, check out the cluster version with
```cassandraql
SELECT cluster_name, listen_address FROM system.local;
```

Now let's create a keyspace and a table inside it
```cassandraql
CREATE KEYSPACE first_keyspace
  WITH REPLICATION = { 
   'class' : 'SimpleStrategy', 
   'replication_factor' : 1 
  };

CREATE TABLE first_keyspace.customers (
    id UUID PRIMARY KEY,
    first_name text,
    second_name text
                                      );
```

Select the data
```cassandraql
select * from first_keyspace.customers ;
```

Insert + select + delete
```cassandraql
INSERT INTO first_keyspace.customers(id, first_name, second_name)
values (af7e23d8-742c-4b24-9b6a-1492e8a570c8, 'Ivan', 'Ivanov');

INSERT INTO first_keyspace.customers(id, first_name, second_name)
values (35430e28-e0a5-41cb-aac5-94275f2d9a1f, 'Petr', 'Petrov');


select * from first_keyspace.customers ;


delete from first_keyspace.customers where id in (af7e23d8-742c-4b24-9b6a-1492e8a570c8, 35430e28-e0a5-41cb-aac5-94275f2d9a1f);
```

## Connection from Java

There are multiple open source drivers listed in [documentation](https://cassandra.apache.org/doc/latest/cassandra/getting_started/drivers.html). 
I chose [datastax](https://github.com/datastax/java-driver).

See the dependencies in `build.gradle`. Navigate to `src/main/java/project4/CassandraRun` for comments on Java client.
