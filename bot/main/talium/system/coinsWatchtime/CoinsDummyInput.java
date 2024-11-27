package talium.system.coinsWatchtime;

import org.jetbrains.annotations.Nullable;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import talium.inputs.Twitch4J.ChatMessage;
import talium.inputs.Twitch4J.TwitchApi;
import talium.system.Out;
import talium.system.coinsWatchtime.chatter.Chatter;
import talium.system.coinsWatchtime.chatter.ChatterService;
import talium.system.inputSystem.BotInput;
import talium.system.inputSystem.Input;
import talium.system.inputSystem.InputStatus;
import talium.system.inputSystem.configuration.InputConfiguration;

import java.util.HashMap;

/**
 * Does nothing, only temporarily exists to add watchtime and coins commands
 */
@Input
public class CoinsDummyInput implements BotInput {
    private static final Logger logger = LoggerFactory.getLogger(CoinsDummyInput.class);
    private static ChatterService chatterService;

    @Autowired
    public void setChatterService(ChatterService chatterService) {
        CoinsDummyInput.chatterService = chatterService;
    }

    @Override
    public void run() {
        //noop
    }

    @Override
    public void shutdown() {
        //noop
    }

    @Override
    public InputStatus getHealth() {
        return InputStatus.HEALTHY;
    }

    @Override
    public String threadName() {
        return "CoinsDummyInput";
    }

    @Override
    public @Nullable InputConfiguration getConfiguration() {
        return new InputConfiguration.Builder()
                .addCallbackCommand("watchtime.getwatchtime", "!watchtime", CoinsDummyInput::triggerGetWatchtime)
                .addCallbackCommand("watchtime.getCoins", "!coins", CoinsDummyInput::triggerGetCoins)
                .addTemplate("coins.coins", "${wt.username} has ${wt.coins} Coins!")
                .addTemplate("coins.watchtime", "${wt.username} has ${wt.daysRounded2} Days of watchtime!")
                .build();
    }

    static class WatchtimeContext {
        String username;
        String coins;
        String watchtimeSeconds;
        String watchtimeHoursRounded;
        String watchtimeHoursRounded2;
        String daysRounded;
        String daysRounded2;

        public WatchtimeContext(Chatter chatter, String username) {
            this.username = username;
            this.coins = String.valueOf(chatter.coins);
            this.watchtimeSeconds = String.valueOf(chatter.watchtimeSeconds);
            this.watchtimeHoursRounded = String.valueOf(Math.round(chatter.watchtimeSeconds / 3600f));
            this.watchtimeHoursRounded2 = String.valueOf(Math.round((chatter.watchtimeSeconds * 100) / 3600f) / 100f);
            this.daysRounded = String.valueOf(Math.round(chatter.watchtimeSeconds / 86400f));
            this.daysRounded2 = String.valueOf(Math.round((chatter.watchtimeSeconds * 100) / 86400f) / 100f);
        }
    }

    public static void triggerGetWatchtime(String triggerId, ChatMessage message) {
        var values = new HashMap<String, Object>();
        var userId = message.user().id();
        var twitchUser = TwitchApi.getUserById(userId);
        if (twitchUser.isEmpty()) {
            logger.warn("Could not get watchtime, no twitch user found for Id: {}", userId);
            return;
        }
        var wt = chatterService.getDataForChatter(userId);
        values.put("wt", new WatchtimeContext(wt, twitchUser.get().getDisplayName()));
        Out.Twitch.sendNamedTemplate("coins.watchtime", values);
    }

    public static void triggerGetCoins(String triggerId, ChatMessage message) {
        var values = new HashMap<String, Object>();
        var userId = message.user().id();
        var twitchUser = TwitchApi.getUserById(userId);
        if (twitchUser.isEmpty()) {
            logger.warn("Could not get watchtime, no twitch user found for Id: {}", userId);
            return;
        }
        var wt = chatterService.getDataForChatter(userId);
        values.put("wt", new WatchtimeContext(wt, twitchUser.get().getDisplayName()));
        Out.Twitch.sendNamedTemplate("coins.coins", values);
    }
}
