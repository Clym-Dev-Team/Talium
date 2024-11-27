package talium.system.twitchCommands.cooldown;

import com.google.common.util.concurrent.ThreadFactoryBuilder;
import kotlin.Pair;
import org.checkerframework.dataflow.qual.Impure;
import org.checkerframework.dataflow.qual.Pure;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import talium.inputs.Twitch4J.ChatMessage;
import talium.inputs.Twitch4J.TwitchUser;

import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.*;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ThreadFactory;
import java.util.concurrent.TimeUnit;

/**
 * This service handles global and user specific message index and time based cooldowns and provides the necessary message indexes to calculate the cooldown state.
 * <br/> <br/>
 *
 */
public class CooldownService {
    //TODO resetable caches

    // User Cooldowns
    /** Map of Twitch User IDs to their last Message Index, increased on each messsage. Used to assign message indexes for each user */
    private static final TreeMap<String, Integer> messageIndexes = new TreeMap<>();
    /** Map of a combination between a triggerId and a TwitchUserID, to the last message Index of the message, that the
     * user used to successfully trigger this command the last time. Updated on successfull command execution */
    private static final HashMap<Pair<String, String>, Integer> messageUserCooldown = new HashMap<>();
    /** Map of a combination between a triggerId and a TwitchUserID, to the Instant of the last message, that the
     * user used to successfully trigger this command the last time. Updated on successfull command execution */
    private static final HashMap<Pair<String, String>, Instant> secondsUserCooldown = new HashMap<>();

    // Global Cooldowns
    /** Current Global message index, updated on each received message. Used to assign global message indexes */
    private static int globalMessageIndex = 0;
    /** Map of triggerId to index of last message that successfully triggered this command */
    private static final HashMap<String, Integer> messageGlobalCooldown = new HashMap<>();
    /** Map of triggerId to instant of last message that successfully triggered this command */
    private static final HashMap<String, Instant> secondsGlobalCooldown = new HashMap<>();

    /** Scheduled cleanup job to reclaim memory from very old userCooldown entries */
    private static final ScheduledExecutorService cleanupService;
    private static final Logger log = LoggerFactory.getLogger(CooldownService.class);

    static {
        ThreadFactory namedThreadFactory = new ThreadFactoryBuilder().setNameFormat("COOLDOWN_CLEANUP_EXECUTOR").build();
        cleanupService = Executors.newSingleThreadScheduledExecutor(namedThreadFactory);
        cleanupService.scheduleAtFixedRate(CooldownService::cleanupCooldownUser, 0, 30, TimeUnit.MINUTES);
    }

    /**
     * Returns the next index for a message of this user.
     * <bold>This function is not pure</bold>, so calling this multiple times per received message will always increase the internal counter in this service.
     * This will cause the chat cooldowns to become inaccurate or behave unexpectedly!
     * <br/> <br/>
     * If for some reason you need to get the current userMessageIndex without increasing the counter, use -> {@link CooldownService#getMessageUserIndex(TwitchUser)}
     *
     * @param user the user for which to increase and get the index
     * @return the new index to use in the received message
     * @apiNote <font color="red">This function increases the internal counter of this service!</font> Only call when you get a new Twitch Message from the specific user!
     * @see CooldownService#getMessageUserIndex(TwitchUser)
     */
    @Impure
    public static int computeMessageUserIndex(TwitchUser user) {
        var mapped = messageIndexes.getOrDefault(user.id(), 0);
        messageIndexes.put(user.id(), mapped + 1);
        return mapped + 1;
    }

    /**
     * Get the messageUserIndex of the most recent message. Does not increase the underlying counter.
     * @param user user from with to get the messageIndex
     * @return messageIndex for most recent message from user
     */
    @Pure
    public static int getMessageUserIndex(TwitchUser user) {
        return messageIndexes.getOrDefault(user.id(), 0);
    }

    /**
     * Returns the next global message index.
     * This index is the index of all messages that were received since startup.
     * <bold>This function is not pure</bold>, so calling this multiple times per received message will always increase the internal counter in this service.
     * This will cause the chat cooldowns to become inaccurate or behave unexpectedly!
     * <br/> <br/>
     * If for some reason you need to get the current globalMessageIndex without increasing the counter, use -> {@link CooldownService#getMessageGlobalIndex()}
     *
     * @return the new global message index to use in the received message
     * @apiNote <font color="red">This function increases the internal counter of this service!</font> Only call when you get a new Twitch chat Message!
     * @see CooldownService#getMessageGlobalIndex()
     */
    @Impure
    public static int computeMessageGlobalIndex() {
        globalMessageIndex++;
        return globalMessageIndex;
    }

    /**
     * Get the messageGlobalIndex of the most recent message out of all users. Does not increase the underlying counter.
     * @return the messageIndex for the most recent messsage for all users
     */
    @Pure
    public static int getMessageGlobalIndex() {
        return globalMessageIndex;
    }

    /**
     * Check if a user is still in userCooldown for a specific trigger.
     * Uses the saved triggerId to lookup data from the last time this triggerId was successfully executed.
     * @param message the ChatMessage that should be checked if it (and the user) is still in cooldown
     * @param triggerId the triggerId
     * @param userCooldown the cooldown settings of the trigger
     * @return if the user is still in cooldown. True means forbidden from executing the trigger
     */
    public static boolean inUserCooldown(ChatMessage message, String triggerId, ChatCooldown userCooldown) {
        if (userCooldown.type == CooldownType.MESSAGES) {
            Integer lastUseMessageId = messageUserCooldown.get(new Pair<>(triggerId, message.user().id()));
            if (lastUseMessageId == null) {
                return false;
            }
            var idDelta = message.userMessageIndex() - lastUseMessageId;
            return idDelta <= userCooldown.amount;
        }
        var lastUseInstant = secondsUserCooldown.get(new Pair<>(triggerId, message.user().id()));
        if (lastUseInstant == null) {
            return false;
        }
        var secondsBetween = ChronoUnit.SECONDS.between(message.sendAT(),lastUseInstant);
        return secondsBetween <= userCooldown.amount;
    }

    /**
     * Check if a trigger is still in globalCooldown. This check is separate from the user cooldown check and is fully user agnostic.
     * You need to separately check if the user itself is in cooldown for this trigger. <br/>
     * Uses the saved triggerId to lookup data from the last time this triggerId was successfully executed.
     * @param message the ChatMessage that should be checked if it is still in cooldown
     * @param triggerId the triggerId
     * @param globalCooldown the cooldown settings of the trigger
     * @return if a user is still in cooldown. True means forbidden from executing the trigger
     */
    public static boolean inGlobalCooldown(ChatMessage message, String triggerId, ChatCooldown globalCooldown) {
        if (globalCooldown.type == CooldownType.MESSAGES) {
            Integer lastUseMessageId = messageGlobalCooldown.get(triggerId);
            if (lastUseMessageId == null) {
                return false;
            }
            int delta = message.globalMessageIndex() - lastUseMessageId;
            return delta <= globalCooldown.amount;
        }
        var lastUseInstant = secondsGlobalCooldown.get(triggerId);
        if (lastUseInstant == null) {
            return false;
        }
        var secondsBetween = ChronoUnit.SECONDS.between(message.sendAT(),lastUseInstant);
        return secondsBetween <= globalCooldown.amount;
    }

    /**
     * Updates the stored information for the last message that executed a trigger. <br/>
     * @param message message was successfully in satisfying all the trigger conditions
     * @param triggerId the triggerId that was satisfied
     * @param globalCooldown the globalCooldown of this trigger
     * @param userCooldown the userCooldown of this trigger
     */
    public static void updateCooldownState(ChatMessage message, String triggerId, ChatCooldown globalCooldown, ChatCooldown userCooldown) {
        if (userCooldown.type == CooldownType.MESSAGES) {
            messageUserCooldown.put(new Pair<>(triggerId, message.user().id()), message.userMessageIndex());
            return;
        } else {
            secondsUserCooldown.put(new Pair<>(triggerId, message.user().id()), Instant.now());
        }

        if (globalCooldown.type == CooldownType.MESSAGES) {
            messageGlobalCooldown.put(triggerId, message.globalMessageIndex());
        } else {
            secondsGlobalCooldown.put(triggerId, Instant.now());
        }
    }

    /**
     * Cleanup task to remove UserCooldown entries that are pretty old
     */
    private static void cleanupCooldownUser() {
        //TODO wrap in try/catch because otherwise executor will end on runtime error
        log.info("Running Cooldown cleanup task");
        // if the index of the message that executed this command last is more than 100 messages ago than the last send message of the user we remove the message
        for (var entry : messageUserCooldown.entrySet()) {
            String userId = entry.getKey().component2();
            var messsageIndexDelta = messageIndexes.get(userId) - entry.getValue();
            if (messsageIndexDelta > 100) {
                messageUserCooldown.remove(entry.getKey());
            }
        }
        // if an entry of the time the user used this command last is more than 60 minutes old, we remove that entry
        for (var entry : secondsUserCooldown.entrySet()) {
            String userId = entry.getKey().component2();
            var timeDelta = ChronoUnit.MINUTES.between(Instant.now(), entry.getValue());
            if (timeDelta > 60) {
                secondsUserCooldown.remove(entry.getKey());
            }
        }
    }

}
