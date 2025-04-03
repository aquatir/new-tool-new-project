# Dockerify java postgres

### Motivation

- Figure out docker-compose with the app inside

## About

Pushing a Javalin app into Docker container, then running it with a database

### Commands

build image
```bash
docker build -t kotlin-javalin-app .
```

run image (don't run normally)
```bash
docker run -p 7070:7070 kotlin-javalin-app
```

run docker-compose with DB 
```bash
docker-compose up
```

