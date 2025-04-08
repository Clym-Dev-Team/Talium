package talium.coinsWatchtime;

import talium.Out;
import talium.Registrar;
import talium.coinsWatchtime.chatter.Chatter;
import talium.coinsWatchtime.chatter.ChatterService;
import talium.twitch4J.ChatMessage;

import java.util.HashMap;

/**
 * Serves all commands related to watchtime and coins
 */
public class WIPWatchtimeCommandServer {
    private static ChatterService chatterService;

    public static void init(ChatterService chatterService) {
        WIPWatchtimeCommandServer.chatterService = chatterService;
        new Registrar.Command("watchtime.getwatchtime", "!watchtime ?.*").registerActionCommand(WIPWatchtimeCommandServer::triggerGetWatchtime);
        new Registrar.Command("watchtime.getCoins", "!coins ?.*").registerActionCommand(WIPWatchtimeCommandServer::triggerGetCoins);
        Registrar.registerTemplate("coins.coins", "${wt.username} has ${wt.coins} Coins!");
        Registrar.registerTemplate("coins.watchtime", "${wt.username} has ${wt.daysRounded2} Days of watchtime!");
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
        var wt = chatterService.getChatterDataOrDefault(message.user().id());
        values.put("wt", new WatchtimeContext(wt, message.user().name()));
        Out.Twitch.sendNamedTemplate("coins.watchtime", values);
    }

    public static void triggerGetCoins(String triggerId, ChatMessage message) {
        var values = new HashMap<String, Object>();
        var wt = chatterService.getChatterDataOrDefault(message.user().id());
        values.put("wt", new WatchtimeContext(wt, message.user().name()));
        Out.Twitch.sendNamedTemplate("coins.coins", values);
    }
}
