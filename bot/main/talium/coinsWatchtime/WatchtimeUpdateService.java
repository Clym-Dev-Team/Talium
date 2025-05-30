package talium.coinsWatchtime;

import com.github.twitch4j.helix.domain.Chatter;
import org.apache.commons.lang3.concurrent.BasicThreadFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import talium.Out;
import talium.coinsWatchtime.chatter.ChatterService;

import java.util.List;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.TimeUnit;

/**
 * Requests the twitch user list (list of active chatters) and adds the gained watchtime and coins for all users.
 */
public class WatchtimeUpdateService {
    private static final Logger logger = LoggerFactory.getLogger(WatchtimeUpdateService.class);

    private static final ScheduledExecutorService CHATTER_UPDATE_SERVICE;

    private static ChatterService chatterService;
    private static final int POLLING_INTERVAL_SECONDS = 60;
    private static final int COIN_PAYOUT_INTERVAL_SECONDS = 600;
    private static final int COIN_PAYOUT_AMOUNT = 1;

    static {
        ThreadFactory namedThreadFactory = new BasicThreadFactory.Builder().namingPattern("CHATTER_UPDATE_EXECUTOR").build();
        CHATTER_UPDATE_SERVICE = Executors.newSingleThreadScheduledExecutor(namedThreadFactory);
    }

    public static void init(ChatterService chatterService) {
        WatchtimeUpdateService.chatterService = chatterService;
        CHATTER_UPDATE_SERVICE.scheduleAtFixedRate(WatchtimeUpdateService::update, 10, POLLING_INTERVAL_SECONDS, TimeUnit.SECONDS);
    }

    private static List<String> getUserList() {
        return Out.Twitch.api.getUserList().stream().map(Chatter::getUserId).toList();
    }

    /**
     * Adds watchtime and coins for each user currently in the chat. </br>
     *
     * Each time the viewer is in the chat, one minute (Polling Interval) is added to the watchtime (and coins payout time).
     * Because we do this check only at a point in time, we accumulate some error with just adding a flat minute.
     * But this error partially cancels out because we would give out to little watchtime in the minute the user leaves.
     * So the maximum error of this approach would be one minute (per chatting session), and 0,5 minutes on average. </br>
     * </br>
     * The coins have a seconds counter like the watchtime. If this counter exceeded the needed seconds to receive a coin,
     * the counter is reset and the coins (equal to the payout amount) are added.
     */
    private static void update() {
        // catch because otherwise the scheduler would end on any runtime error
        try {
            logger.info("Refreshing user list");
            if (!Out.Twitch.api.isOnline()) {
                logger.debug("Channel online: false");
                return;
            }
            logger.debug("Channel online: true");
            var chatters = chatterService.getChattersOrDefault(getUserList());
            for (var user : chatters) {
                user.watchtimeSeconds += POLLING_INTERVAL_SECONDS;
                user.secondsSinceLastCoinsGain += POLLING_INTERVAL_SECONDS;

                if (user.secondsSinceLastCoinsGain >= COIN_PAYOUT_INTERVAL_SECONDS) {
                    user.secondsSinceLastCoinsGain = 0;
                    user.coins += COIN_PAYOUT_AMOUNT;
                }
            }
            chatterService.saveAll(chatters);
        } catch (Exception e) {
            logger.error("Failed to refresh user list", e);
        }
    }
}
