## Builder
FROM openjdk:21-jdk-slim AS builder

COPY . .
RUN chmod +x gradlew  \
    && ./gradlew clean shadowJar --no-daemon

## Runner
FROM openjdk:21-jdk-slim
WORKDIR /app
COPY --from=builder /app/build/libs/app-all.jar .

# Expose relevant application port
EXPOSE 7070

# Run the application
CMD ["java", "-jar", "app-all.jar"]

