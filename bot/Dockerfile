FROM maven:3.9.9-eclipse-temurin-21 as builder

WORKDIR /talium
COPY pom.xml pom.xml
COPY main main

RUN mvn -f pom.xml clean package -DskipTests

FROM openjdk:21-jdk-slim

COPY --from=builder ./talium/target/TaliumBot-1.jar /home/Talium.jar
COPY /resources/logback.xml /home/config/logback.xml
COPY /resources/application.properties /home/config/application.properties
COPY /resources/application-example.properties /home/config/application-prod.properties

EXPOSE 4771
ENTRYPOINT ["java", "--enable-preview", "-Dspring.profiles.active=prod", "-Dspring.config.location=file:/home/config/", "-Dlogging.config=/home/config/logback.xml", "-jar", "/home/Talium.jar" ]