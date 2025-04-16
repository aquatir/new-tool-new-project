## Builder
FROM openjdk:21-jdk-slim AS builder

COPY . .
RUN chmod +x gradlew  \
    && ./gradlew clean shadowJar --no-daemon

## Runner
FROM openjdk:21-jdk-slim
WORKDIR /app
COPY --from=builder /app/build/libs/app-all.jar .

RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*

# Expose relevant application port
EXPOSE 7070

# Run the application
CMD ["java", "-jar", "app-all.jar"]

