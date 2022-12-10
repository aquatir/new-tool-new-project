# Debezium

A Change Data Capture (CDC) tool build on top of Apache Kafka.

## Links

- Web site: https://debezium.io
- Official tutorial link: https://debezium.io/documentation/reference/2.0/tutorial.html

## Week 1 — Debezium

### Motivation

- Level 0: understand what debezium is. Configure minimal working example.
- Level 1: configure application
  using [outbox pattern](https://microservices.io/patterns/data/transactional-outbox.html)
  with Debezium.

### Links

- [intro.md](./docs/intro.md). Setting up `Zookeeper`, `Kafka`, `MySQL` and `KafkaConnect` to database via command line
  in multiple terminals.
- [java-simple-app.md](./docs/java-simple-app.md). *WORK IN PROGRESS*: Java application with `docker compose` setup for
  debezium. It uses listener to read debezium events to update it's internal state.
- [java-app-outbox.md](./docs/java-app-outbox.md). *WORK IN PROGRESS*: Java application with `docker compose` setup for
  debezium that exposes two apps: one produces events using outbox pattern and another reads those events. Uses
  persistent storage for database and kafka
