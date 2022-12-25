# Intro

## 

Running Cassandra and connecting via Java and console

Navigate to `docker` directory and run
```shell
docker-compose up
```

Wait until the line
```
Starting listening for CQL clients on /0.0.0.0:9042 (unencrypted).
```

And than run `cqlsh` console
```shell
docker exec -it cass_cluster cqlsh
```

You can check out Cassandra's default config 
```shell
docker exec -it cass_cluster /bin/bash
cd /etc/cassandra
cat cassandra.yaml
```

There are a lot of different configs apart from `cassandra.yaml`. 
