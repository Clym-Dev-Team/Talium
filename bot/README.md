# Build with Docker

```shell
docker image build --tag [AUTHOR OR REPO NAME]/talium:[VERSION] .
```

# Docker compose
> (Given are the fully qualified paths, with filenames, form the repo root, or docker compose project root)

After you have pushed to docker image to a image registry, or build it on the machine you wanna use it, copy the:
- copy example Docker compose file (and rename it to docker-compose.yml)
- create `config` folder next to the docker-compose.yml file
- copy `/bot/resources/application.properties` to `/config/application.properties`
- copy `/bot/resources/logback.xml` to `/config/logback.xml`
- copy and rename `/bot/resources/application-example.properties` to `/config/application-prod.properties`
- in `application-prod.properties` fill all empty values, and replace incorrect once (like the location of your database)
- make sure you have `allowedPanelUsers` set to at least one twitch account name, otherwise no one will have access to the panel
- make sure `disableAllAuth` is false
- make sure the baseUrls are set to the ip/domain you will be using in the browser
- start the container: `docker compose start -d`

In the end, it should look like this: 
  ├── docker-compose.yml 
  ├── config/
      ├── application.properties
      ├── application-prod.properties
      ├── logback-spring.xml