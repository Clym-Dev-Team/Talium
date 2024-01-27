package main.inputs.Twitch4J;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.philippheuer.events4j.simple.SimpleEventHandler;
import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.auth.providers.TwitchIdentityProvider;
import com.github.twitch4j.chat.events.TwitchEvent;
import com.github.twitch4j.helix.TwitchHelix;
import main.inputs.shared.oauth.OAuthEndpoint;
import main.inputs.shared.oauth.OauthAccount;
import main.system.eventSystem.EventDispatcher;
import main.system.inputSystem.Input;
import main.system.inputSystem.TwitchBotInput;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;

import java.util.ArrayList;
import java.util.Optional;

@Input
@Component
public class Twitch4JInput implements TwitchBotInput {

    private static final String channelName = "clym";
    private static final String chatAccountName = "orciument";

    //"CONFIG"
    private static String app_clientID = "";
    @Value("${twitchApp_ID}")
    public void setApp_clientID(String app_clientID) {
        Twitch4JInput.app_clientID = app_clientID;
    }

    private static String app_clientSecret = "";
    @Value("${twitchApp_Secret}")
    public void setApp_clientSecret(String app_clientSecret) {
        Twitch4JInput.app_clientSecret = app_clientSecret;
    }

    private static final Logger logger = LoggerFactory.getLogger(Twitch4JInput.class);

    private static TwitchIdentityProvider iProvider;
    private OAuth2Credential oAuth2Credential;
    private TwitchClient twitchClient;
    private boolean running = false;
    public static TwitchHelix broadCasterHelix;
    //GraphQL is disabled until we need it, because the module is not yet finished
    //public static TwitchGraphQL broadCasterGraphQL;

    public Twitch4JInput() {
    }

    @Override
    public boolean checkConfiguration() {
        //TODO fehlen einige Config Tests
        if (OauthAccount.repo == null) {
            logger.error("TwitchAccountRepository is null!");
            return false;
        }
//        if (!OauthAccount.repo.existsByAccName("primary")) {
//            logger.error("No OAuth Credentials found!");
//            return false;
//        }
        //TODO iProvider.isCredentialValid() is valid
        logger.info("Configuration and Credentials found and configured correctly");
        return true;
    }

    @Override
    public void run() {
        logger.debug("Starting... ");
        iProvider = new TwitchIdentityProvider(app_clientID, app_clientSecret, OAuthEndpoint.getRedirectUrl("twitch"));

        var creds = getRefreshedOauthFromDB(iProvider);
        if (creds.isEmpty()) {
            logger.warn("Twitch credentials could not be found or refreshed, waiting for new Oauth Token to be created!");
            logger.warn("Head to " + OAuthEndpoint.getOauthSetupUrl() + " to Setup a new Oauth connection");
            creds = createNewOauth();
            if (creds.isEmpty()) {
                logger.error("Could neither load old credentials, nor create new once, aborting startup!");
                return;
            }
            logger.warn("Created new Oauth. Warning resolved!");
        }
        oAuth2Credential = creds.get();

        TwitchClient twitchClient = TwitchClientBuilder.builder()
                .withEnableHelix(true)
                .withEnableChat(true)
                //GraphQL is disabled until we need it, because the module is not yet finished
                //.withEnableGraphQL(true)
                .withEnablePubSub(true)
                .withDefaultAuthToken(oAuth2Credential)
                .withChatAccount(oAuth2Credential)
                .withDefaultEventHandler(SimpleEventHandler.class)
                .build();
        twitchClient.getClientHelper().enableStreamEventListener(channelName);
        twitchClient.getChat().joinChannel(channelName);
//        twitchClient.getEventManager().onEvent(ChannelMessageEvent.class, EventDispatcher::dispatch);
        twitchClient.getEventManager().onEvent(TwitchEvent.class, EventDispatcher::dispatch);

        this.twitchClient = twitchClient;

        //GraphQL is disabled until we need it, because the module is not yet finished
        //broadCasterGraphQL = twitchClient.getGraphQL();
        broadCasterHelix = twitchClient.getHelix();
        logger.debug("Start successful!");
        running = true;
    }

    @Override
    public boolean running() {
        return running;
    }

    @Override
    public boolean shutdown() {
        OauthAccount account = new OauthAccount(oAuth2Credential.getUserId(), oAuth2Credential.getRefreshToken(), chatAccountName);
        OauthAccount.repo.save(account);
        twitchClient.close();
        logger.debug("Shutdown successful!");
        running = false;
        return true;
    }

    @Override
    public String threadName() {
        return "TwitchReading";
    }

    private Optional<OAuth2Credential> getRefreshedOauthFromDB(TwitchIdentityProvider iProvider) {
        Optional<OauthAccount> dbCreds = OauthAccount.repo.getByAccName(chatAccountName);
        if (dbCreds.isEmpty()) {
            return Optional.empty();
        }

        // We can leave all the other once emphty because refreshCredential will fill them in for us
        return iProvider.refreshCredential(new OAuth2Credential(
                "",
                "",
                dbCreds.get().refreshToken,
                "",
                "",
                0,
                null
        ));
    }

    private Optional<OAuth2Credential> createNewOauth() {
        var scopes = new ArrayList<>();
        scopes.add("chat:edit");
        scopes.add("chat:read");
        Optional<String> code = OAuthEndpoint.newOAuthGrantFlow(chatAccountName, "twitch", iProvider, scopes);
        if (code.isEmpty()) {
            return Optional.empty();
        }
        return Optional.of(iProvider.getCredentialByCode(code.get()));
    }
}
