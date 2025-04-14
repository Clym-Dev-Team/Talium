# Build with Docker

The Panel and Bot are build into separate images. The images should be relatively agnostic to the setup around them.

Generally, configuration data is saved outside the container in a folder /config on the host, that is mapped into both containers via a docker compose file. 

Besides some keys, hosts (domains mostly), and some basic settings the configuration in the example config and docker-compose files should just work.

The **Bot** is just compiled and packaged using maven, with a few options to make the build go a bit faster, or cache better.
(The tests are skipped because getting all the secrets into the container just to execute the tests is to much work, and the pipeline should have tested the code long bevor trying to build the container)

The **Panel** is build using vite and tsc, but without copying the .env files used to provide config data while running in the local dev environment. 
That way the values aren't baked in. The Resulting bundle is then served via an NginX Server. This Server pulls tripple duty:  
it serves a special config file so that the panel can know where to contact the backend bot, it serves the index.html and other assets for the panel,
and it redirects any traffic on a specific url path (/backend) to the actual address of the backend. That way, the frontend and backend services 
can both be exposed via a single port on the Nginx Panel/Proxy server. 

# Build images locally
A convenience script was developed to build both images locally with the same tag. This script automatically sets all
the build arguments (that can reasonably be guessed).

**Execute with WSL on Windows or on Linux:**
```shell
bash ./build-docker.sh [AUTHOR OR REPO NAME] [VERSION/TAG NAME] [OPTIONAL: git commit sha]
```
If you have a default distro set for your wls (`wsl --set-default DISTRO`), this should just run without entering wls explicitly.

# Build in CI
The CI should do basically the same as the shell script. We are not using the shell script directly, 
because we can better see what is going on in the pipeline, if it is not hidden in an extra script. 
It also allows us some more options relating to handling failures.

# Docker compose setup
This section covers how the different images need to setup, and what configuration data you need to set.

To deploy the bot using docker (compose):
1. copy the example docker-compose.yml
2. copy the config directory.
3. You directory should now look something like this:
   ```
   ├── docker-compose.yml
   ├── config/
       ├── application-prod.properties
       ├── logback-spring.xml
       ├── public.properties
       ├── nginx.conf
   ```
   The nginx.conf and logback-spring.xml should just be ready, without you needing to change anything. 
   They are still manually mounted on the outside of the container, so that you can easily change values and just restart
   the container. 
4. If you want to use your locally build images, you need to change the names of the images in the docker-compose.yml file
5. Figure out what address/IP you will use to connect to the panel. You will need to use HTTPS to connect, because of the Twitch Oauth procedure.
   An Example Address could be `https://talium.exampleDomain`
6. If you are using the default setup, the url to connect to the actual Bot is `YOUR_PANEL_ADDRESS/backend`. 
   If you are using a custom setup, you will figure out the address you will use for the Bot, this will also need to use HTTPS.
7. Go to dev.twitch.tv login with twitch and register an Application for the Bot. Select `Chat Bot` as the category. Under `Client Types` select `confidential`. Set the redirect url to `YOUR_BACKEND_URL/auth/twitch`.
   In your default example this would be `https://talium.exampleDomain/backend/auth/twitch`. And copy the clientId and clientSecret for later
8. Go to dev.twitch.tv login with twitch and register an Application for the Panel. Select `Website Integration` as the category. Under `Client Types` select `public`. Set the redirect url to `YOUR_PANEL_URL/twitchToken`.
   In your default example this would be `https://talium.exampleDomain/twitchToken`. And copy the clientId for later
9. You now need to configure the `public.properties` file
   This manly involves getting the secrets for the twitch Oauth process, and selecting the twitch channels to use.
    - Open `public.properties` and replace `http://localhost:9772` in `panelBaseUrl` and `serverBaseUrl` with your URLs from earlier to connect to the panel and bot (server) respectively.
      This step is necessary so that the panel can then use this url as a redirect after the Twitch Oauth process is complete.
    - Replace the `twitchClientId` in `public.properties` with the clientId of your twitch dev application for the panel from earlier
10. You now need to configure the `application-prod.properties` file
    - Set the same `panelBaseUrl` and `serverBaseUrl` properties to the same values as in `public.properties`
    - Update the Database password in and update the other database fields if you used a different database
    - Set `allowedPanelUsers` to your twitch username, and make sure `overwritePanelUsers` is set to true.
      This allows you user to use the panel without you having permissions in the database.
    - Set the `twitchAppId` and `twitchAppSecret` to the values from the twitch dev console app for the Bot from earlier
    - Set the `twitchBotAccountName` to the name of the twitch account you want to use to read and write into the chat. This account 'will become the bot'
    - Set the `twitchChannelName` (this is the channel the bot reads from) and the `twitchOutputToChannel` (this is the channel the bot writes to)
      In most cases this will be the same channel name. But the channels can be different, this is usefull for testing without you chat noticing.
    - Set the `tipeeeApikey` and `tipeeeChannel`. You get the tipeeeApikey from the developer section of your TipeeeStream Dashboard
11. Open a console in the directory that contains the docker-compose.yml
12. use `docker compose up` to start the containers (optionally with the flag `-d` to run in the background)
13. After all containers are running, use you PANEL_URL in a browser to access the panel
14. If it is the first time you are logging in to this bot, twitch will redirect you to a page to give the Panel application from the earlier twitch dev console some access to your account. 
    Grant it this access. If you have configured the `panelBaseUrl` and `serverBaseUrl` correctly you should now be redirected to the panel.
15. Go to the Panel User Page and enter the Twitch Usernames of all people that should have access to the panel. 
16. Switch to a browser in which you are logged in with the account that should be used as the bot account
17. Go to the Oauth Accounts Page, and click the Button to Authorize the Bot application from the twitch dev console to do various actions in the name of this twitch account. 
18. Finally, you should set the `overwritePanelUsers` property in `application-prod.properties` to `false`, and restart the Bot.
19. You should now be done

TODO point out that application.properties


# TLS/HTTPS Setup
While Talium does not handle HTTPS traffic directly, an easy solution is to use something like `nginx-proxy-manager` 
to automatically provision a TLS certificate, and terminate the TLS at this (ingress) proxy bevor sending the traffic through to the talium panel nginx.
