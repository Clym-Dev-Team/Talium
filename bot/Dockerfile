FROM maven:3.9.9-eclipse-temurin-21 as builder

ARG TAG_NAME

WORKDIR /talium

COPY pom.xml pom.xml

RUN mvn -f pom.xml verify --fail-never

COPY main main

RUN mvn -f pom.xml package -DskipTests -Drevision="${TAG_NAME}"

FROM openjdk:21-jdk-slim

ARG COMMIT_SHA
ARG TAG_NAME
ARG CREATED_AT

LABEL COMMIT_SHA=$COMMIT_SHA
LABEL VERSION=$TAG_NAME
LABEL CREATED_AT=$CREATED_AT

COPY --from=builder ./talium/target/TaliumBot-${TAG_NAME}.jar /home/Talium.jar
COPY /resources/logback.xml /home/config/logback.xml
COPY /resources/application.properties /home/config/application.properties

# other config files are not copied in, they are later mounted into the container

EXPOSE 4771

ENV IMAGE_COMMIT_SHA=$COMMIT_SHA
ENV IMAGE_VERSION=$TAG_NAME
ENV IMAGE_CREATED_AT=$CREATED_AT
ENV PLATFORM=DOCKER

ENTRYPOINT ["java", "--enable-preview", "-Dspring.profiles.active=prod", "-Dspring.config.location=file:/home/config/", "-Dlogging.config=/home/config/logback.xml", "-jar", "/home/Talium.jar" ]