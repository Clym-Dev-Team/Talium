package talium.twitch4J;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.philippheuer.events4j.simple.SimpleEventHandler;
import com.github.twitch4j.TwitchClient;
import com.github.twitch4j.TwitchClientBuilder;
import com.github.twitch4j.auth.providers.TwitchIdentityProvider;
import com.github.twitch4j.chat.TwitchChat;
import com.github.twitch4j.chat.events.TwitchEvent;
import com.github.twitch4j.chat.events.channel.ChannelMessageEvent;
import com.github.twitch4j.helix.TwitchHelix;
import org.apache.commons.lang.RandomStringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import talium.Out;
import talium.Registrar;
import talium.eventSystem.EventDispatcher;
import talium.eventSystem.Subscriber;
import talium.inputSystem.BotInput;
import talium.inputSystem.HealthManager;
import talium.inputSystem.InputStatus;
import talium.oauthConnector.OAuthEndpoint;
import talium.oauthConnector.OauthAccount;
import talium.oauthConnector.OauthAccountRepo;

import java.text.SimpleDateFormat;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Optional;

public class Twitch4JInput implements BotInput {

    private static final Logger logger = LoggerFactory.getLogger(Twitch4JInput.class);

    private TwitchConfig config;
    private OauthAccountRepo oauthRepo;

    protected static volatile TwitchChat chat;
    protected static volatile TwitchHelix helix;
    protected static OAuth2Credential oAuth2Credential;
    private TwitchIdentityProvider iProvider;
    private TwitchClient twitchClient;

    protected static String broadCasterChannelId;

    public Twitch4JInput(TwitchConfig config, OauthAccountRepo oauthRepo) {
        this.config = config;
        this.oauthRepo = oauthRepo;
    }


    @Override
    public void startup() {
        logger.debug("Starting... ");
        Registrar.registerHealthDescription(Twitch4JInput.class, "Twitch4JInput", "");
        HealthManager.reportStatus(Twitch4JInput.class, InputStatus.STARTING);
        if (!config.hasChannelName()) {
            logger.warn("Using twitchChannelName as botAccountName: {}", config.channelName());
            logger.warn("Consider setting the botAccountName explicitly to avoid accidental misconfiguration.");
            config = config.setAccountName(config.channelName());
        }
        if (!config.hasSendTo()) {
            logger.warn("Using twitchChannelName as twitchOutputToChannel: {}", config.channelName());
            logger.warn("Consider setting the twitchOutputToChannel explicitly to avoid accidental misconfiguration.");
            config = config.setSendTo(config.channelName());
        }

        iProvider = new TwitchIdentityProvider(config.app_clientID(), config.app_clientSecret(), OAuthEndpoint.getRedirectUrl("twitch"));

        var creds = getRefreshedOauthFromDB(iProvider);
        if (creds.isEmpty()) {
            logger.warn("Twitch credentials could not be found or refreshed, waiting for new Oauth Token to be created!");
            logger.warn(STR."Head to \{OAuthEndpoint.getOauthSetupUrl()} to Setup a new Oauth connection");
            HealthManager.reportStatus(Twitch4JInput.class, InputStatus.INJURED);
            creds = createNewOauth();
            //check if still empty after oauth
            if (creds.isEmpty()) {
                logger.error("Could neither load old credentials, nor create new once, aborting input startup!");
                HealthManager.reportStatus(Twitch4JInput.class, InputStatus.DEAD);
                return;
            } else {
                logger.warn("Created new Oauth. Warning resolved!");
                HealthManager.reportStatus(Twitch4JInput.class, InputStatus.STARTING);
            }
        }
        oAuth2Credential = creds.get();

        Optional<Boolean> credentialValid = iProvider.isCredentialValid(oAuth2Credential);
        if (credentialValid.isEmpty()) {
            logger.error("Failed to check if credential is valid");
            HealthManager.reportStatus(Twitch4JInput.class, InputStatus.DEAD);
            return;
        }
        if (!credentialValid.get()) {
            logger.warn("Oauth credential is not valid");
            Optional<OAuth2Credential> newCredential = iProvider.refreshCredential(oAuth2Credential);
            if (newCredential.isEmpty()) {
                logger.error("Failed to refresh credential for unknown reason");
                HealthManager.reportStatus(Twitch4JInput.class, InputStatus.DEAD);
                return;
            }
            oAuth2Credential = newCredential.get();
        }
        if (iProvider.getAdditionalCredentialInformation(oAuth2Credential).isEmpty()) {
            logger.warn("Oauth additional credentials could not be found: {}", oAuth2Credential);
            HealthManager.reportStatus(Twitch4JInput.class, InputStatus.DEAD);
            return;
        }

        logger.info("Credential, receivedAt: {}, expiresIn: {}", oAuth2Credential.getReceivedAt(), oAuth2Credential.getExpiresIn());
        TwitchClient twitchClient = TwitchClientBuilder.builder()
                .withClientId(config.app_clientID())
                .withClientSecret(config.app_clientSecret())
                .withRedirectUrl(OAuthEndpoint.getRedirectUrl("twitch"))
                .withEnableHelix(true)
                .withEnableChat(true)
                .withDefaultAuthToken(oAuth2Credential)
                .withChatAccount(oAuth2Credential)
                .withDefaultEventHandler(SimpleEventHandler.class)
                .build();
        twitchClient.getClientHelper().enableStreamEventListener(config.channelName());
        twitchClient.getChat().joinChannel(config.channelName());
        twitchClient.getEventManager().onEvent(TwitchEvent.class, EventDispatcher::dispatch);

        this.twitchClient = twitchClient;
        chat = twitchClient.getChat();
        helix = twitchClient.getHelix();
        broadCasterChannelId = helix
                .getUsers(null, null, List.of(config.channelName()))
                .execute()
                .getUsers()
                .getFirst()
                .getId();
        logger.debug("Start successful!");
        HealthManager.reportStatus(Twitch4JInput.class, InputStatus.HEALTHY);
        Out.Twitch.api = new TwitchApiImpl(helix, chat, config.sendTo());
    }

    @Override
    public void shutdown() {
        if (oAuth2Credential != null) {
            OauthAccount account = new OauthAccount(config.chatAccountName(), "twitch", oAuth2Credential.getRefreshToken());
            oauthRepo.save(account);
        }
        if (twitchClient != null) {
            twitchClient.close();
        }
        logger.debug("Shutdown successful!");
    }

    private Optional<OAuth2Credential> getRefreshedOauthFromDB(TwitchIdentityProvider iProvider) {
        Optional<OauthAccount> dbCreds = oauthRepo.getByAccNameAndService(config.chatAccountName(), "twitch");
        if (dbCreds.isEmpty()) {
            return Optional.empty();
        }
        // We can leave all the other once empty because refreshCredential will fill them in for us
        return iProvider.refreshCredential(new OAuth2Credential("", "", dbCreds.get().refreshToken, "", "", 0, null));
    }

    private Optional<OAuth2Credential> createNewOauth() {
        String authorizationServer = "https://id.twitch.tv/oauth2/authorize";
        var scopes = new ArrayList<String>();
        scopes.add("chat:edit");
        scopes.add("chat:read");
        scopes.add("moderator:read:chatters");

        // park scopes that could be needed in the future
        //scopes.add("moderator:manage:chat_settings"); // change emote-only & follow-only

        //scopes.add("channel:manage:broadcast"); // if we want to change the stream info
        //scopes.add("channel:manage:redemptions"); // probably needed if we automatically want to accept a channel point redemption for triggering a command (channel:read:redemptions)

        // for overlay
        //scopes.add("channel:read:polls"); // if we want to access polls
        //scopes.add("channel:read:predictions"); // if we want to access predictions
        //scopes.add("channel:read:hype_train"); // if we need to read information about the current hypetrain

        String state = RandomStringUtils.randomAlphanumeric(30);
        String auth_url = String.format("%s?response_type=%s&client_id=%s&redirect_uri=%s&scope=%s&state=%s",
                authorizationServer,
                "code",
                config.app_clientID(),
                OAuthEndpoint.getRedirectUrl("twitch"),
                String.join(" ", scopes),
                state
        );
        Optional<String> code = OAuthEndpoint.newAuthRequest(config.chatAccountName(), "twitch", auth_url, state);
        return code.map(s -> iProvider.getCredentialByCode(s));
    }

    private static final SimpleDateFormat simpleDateFormat = new SimpleDateFormat("DD-HH:mm:ss.SSS");

    @Subscriber
    public static void convertTwitchMessage(ChannelMessageEvent messageEvent) {
        if (messageEvent.getMessage().toCharArray()[0] == '!') {
            System.out.println(simpleDateFormat.format(new Date()) + " |CHAT | " + messageEvent.getUser().getName() + ": " + messageEvent.getMessage());
        }
        try {
            EventDispatcher.dispatch(ChatMessage.fromChannelMessageEvent(messageEvent));
        } catch (ChatMessage.ChatMessageMalformedExceptions e) {
            logger.error("Error converting Twitch message", e);
        }
    }
}