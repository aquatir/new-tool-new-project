# Req Resp Capture

Create a Spring Cloud Gateway project that will

1. Log request-response pair fully from the backends
2. Receive headers from the backends and extra information from them, dropping the headers
3. Send request-responses to Kafka topic

## Project 9 — Req Resp Capture

### Motivation

- Fun building exercise

## About

Run both apps with this in different (or same) console

```bash
./gateway/.gradlew bootRun
./backend/.gradlew run
```

Run Kafka
```bash
docker compose -f kafka.yaml up 
```

Start reading messages with `kcat` (install it first if needed)
```
kcat -C -b localhost:9092 -t topic
```

Open `gateway/readme.md` to see example requests. Look at gateway logs to see what it logs

