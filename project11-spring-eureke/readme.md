# Eureke 

Eureke Service Registry with Spring

https://github.com/Netflix/eureka

## Motivation

- Level 0: understand how Eureke works
- Level 1: call services using Eureke discovery

### How it works

1. Run eureka server by navigating into `eureka-server` folder and running `./mvnw spring-boot:run`
2. Run apps by running `./mvnw spring-boot:run` in their respective folders;
3. Navigate to `localhost:8761` to see Eureke dashboard.
4. Run `curl localhost:8082/helloEureka` to call `serviceb` which will discover `serviea` from Eureka

Turn off Eureka server and run `curl` again. It should work, because Eureka client caches the discovered services.


