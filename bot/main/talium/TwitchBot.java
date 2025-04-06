package talium;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import jakarta.annotation.PreDestroy;
import jakarta.persistence.PreRemove;
import org.apache.commons.lang3.concurrent.BasicThreadFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.ConfigurableApplicationContext;
import org.springframework.data.jpa.repository.config.EnableJpaRepositories;
import talium.coinsWatchtime.WIPWatchtimeCommandServer;
import talium.coinsWatchtime.WatchtimeUpdateService;
import talium.coinsWatchtime.chatter.ChatterService;
import talium.giveaways.GiveawayService;
import talium.giveaways.persistence.GiveawayRepo;
import talium.inputSystem.BotInput;
import talium.inputSystem.HealthManager;
import talium.inputSystem.InputStatus;
import talium.oauthConnector.OauthAccountRepo;
import talium.tipeeeStream.DonationRepo;
import talium.tipeeeStream.TipeeeConfig;
import talium.tipeeeStream.TipeeeInput;
import talium.twitch4J.Twitch4JInput;
import talium.twitch4J.TwitchConfig;
import talium.twitchCommands.triggerEngine.TriggerProvider;

import java.time.Instant;

import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.TimeUnit;

@SpringBootApplication
@EnableJpaRepositories
public class TwitchBot {

    public static boolean requestedShutdown = false;
    private static final Logger logger = LoggerFactory.getLogger(TwitchBot.class);
    public static final Gson gson = new GsonBuilder()
            .serializeNulls()
            .registerTypeAdapter(Instant.class, new InstantTypeAdapter())
            .create();

    public static void main(String[] args) {
        startup();
    }

    private static final ScheduledExecutorService TWITCH4J_PERIODIC_RESTART_EXECUTOR;

    static {
        ThreadFactory namedThreadFactory = new BasicThreadFactory.Builder().namingPattern("TWITCH4J_PERIODIC_RESTART_EXECUTOR").build();
        TWITCH4J_PERIODIC_RESTART_EXECUTOR = Executors.newSingleThreadScheduledExecutor(namedThreadFactory);
    }

    private static ConfigurableApplicationContext ctx;
    private static BotInput twitch;
    private static BotInput tipeee;

    public static void startup() {
        StopWatch time = new StopWatch(StopWatch.TYPE.STARTUP);
        ctx = SpringApplication.run(TwitchBot.class);

        System.out.println();
        System.out.println("DateFormat: DayNumber-Hour:Minute:Second:Millis");
        System.out.println("DDD-HH:mm:ss.SSS |LEVEL| [THREAD]        LOGGER (Source Class)               - MSG");
        System.out.println("-----------------|-----|-[-------------]---------------------------------------------------------------------------------------------------------------------------------------------");
        var serverPort = ctx.getEnvironment().getProperty("server.port");
        logger.info("Server started on Port: {}", serverPort);

        // Start initializing system components, the order of operations is very important!
        twitch = startInput(new Twitch4JInput(
                ctx.getBean(TwitchConfig.class),
                ctx.getBean(OauthAccountRepo.class)
        ));
        tipeee = startInput(new TipeeeInput(
                ctx.getBean(TipeeeConfig.class),
                ctx.getBean(DonationRepo.class))
        );

        // This section is used to pass the execution/control to different parts of the bot to do some initialisation
        WatchtimeUpdateService.init(ctx.getBean(ChatterService.class));
        WIPWatchtimeCommandServer.init(ctx.getBean(ChatterService.class));
        GiveawayService.init(ctx.getBean(GiveawayRepo.class), ctx.getBean(ChatterService.class));

        TriggerProvider.rebuildTriggerCache();
        //TODO remove all templates that were once registered automatically, but are no longer

        time.close();

        TWITCH4J_PERIODIC_RESTART_EXECUTOR.scheduleAtFixedRate(() -> {
            logger.info("Starting periodic Twitch reconnect");
            reconnectTwitch();
        }, 120, 120, TimeUnit.MINUTES);
    }

    @PreDestroy
    @PreRemove
    public static void shutdown() {
        StopWatch time = new StopWatch(StopWatch.TYPE.SHUTDOWN);
        requestedShutdown = true;
        stopInput(twitch);
        stopInput(tipeee);
        time.close();
    }

    private static BotInput startInput(BotInput input) {
        try {
            input.startup();
        } catch (Exception e) {
            logger.error("Exception starting Input: {} because: {}", input.getClass().getCanonicalName(), e.getMessage());
            HealthManager.reportStatus(input.getClass(), InputStatus.DEAD);
        }
        return input;
    }

    private static void stopInput(BotInput input) {
        try {
            input.shutdown();
        } catch (Exception e) {
            logger.error("Exception stopping Input: {} because: {}", input.getClass().getCanonicalName(), e.getMessage());
            HealthManager.reportStatus(input.getClass(), InputStatus.DEAD);
        }
    }

    public static boolean reconnectTwitch() {
        try {
            twitch.shutdown();
            var twitch = new Twitch4JInput(
                    ctx.getBean(TwitchConfig.class),
                    ctx.getBean(OauthAccountRepo.class)
            );
            twitch.startup();
            TwitchBot.twitch = twitch;
            return true;
        } catch (Exception e) {
            logger.error("Exception reconnecting twitch: {}", e.getMessage());
            return false;
        }
    }
}
