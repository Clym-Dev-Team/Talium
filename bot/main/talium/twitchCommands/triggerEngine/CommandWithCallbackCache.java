package talium.twitchCommands.triggerEngine;

import org.jetbrains.annotations.NotNull;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.lang.Nullable;
import talium.twitchCommands.persistence.CommandEntity;
import talium.twitchCommands.persistence.TriggerService;

import java.util.HashMap;
import java.util.List;
import java.util.Objects;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

public class CommandWithCallbackCache {
    private static final Logger log = LoggerFactory.getLogger(CommandWithCallbackCache.class);

    /**
     * Cache of runtime triggers, should only be build from the db, and needs to be rebuilt on trigger change
     */
    private static List<RunnableCommand> triggers;
    /// Permanent list of all callbacks that are registered. This can not be rebuilt, only singular callbacks will be removed, if they are unregistered
    private static final HashMap<String, TriggerCallback> callbackMap = new HashMap<>();
    private static TriggerService triggerService;

    public static void init(TriggerService triggerService) {
        CommandWithCallbackCache.triggerService = triggerService;
        triggers = triggerService
                .getAllUserTriggers()
                .stream()
                .map(CommandWithCallbackCache::commandEntityToRunnable)
                .filter(Objects::nonNull)
                .collect(Collectors.toList());
    }

    public static List<RunnableCommand> triggers() {
        return triggers;
    }


    /**
     * Rebuilds the cache of triggers
     */
    public static void rebuildTriggerCacheFromDB() {
        triggers = triggerService
                .getAllTriggers()
                .stream()
                .map(CommandWithCallbackCache::commandEntityToRunnable)
                .filter(Objects::nonNull)
                .collect(Collectors.toList());
    }

    /**
     * Transforms a CommandEntity into a runnable Command for use of the triggerEngine.
     *
     * @param entity command to transform
     * @return transformed command or null, if no template or callback exists for this command
     * @apiNote Discards all disabled patterns
     */
    @Nullable
    private static RunnableCommand commandEntityToRunnable(CommandEntity entity) {
        var regexes = entity.patterns
                .stream()
                .filter(messagePattern -> messagePattern.isEnabled)
                .map(pattern -> {
                    if (pattern.isRegex) {
                        return Pattern.compile(pattern.pattern);
                    } else {
                        return compilePrefixPattern(pattern.pattern);
                    }
                }).toList();
        var callbackFromMap = callbackMap.get(entity.id);
        if (entity.template == null && callbackFromMap == null) {
            log.warn("Command: {} has no template or callback registered. It cannot be executed", entity.id);
            return null;
        }
        return new RunnableCommand(
                entity.id,
                regexes,
                entity.permission,
                entity.userCooldown,
                entity.globalCooldown,
                callbackFromMap != null ? callbackFromMap : TriggerEngine.TEXT_COMMAND_CALLBACK
        );
    }

    private static @NotNull Pattern compilePrefixPattern(String prefixPattern) {
        return Pattern.compile(STR."^\{prefixPattern}(?: |$).*", Pattern.CASE_INSENSITIVE);
    }

    /**
     * Save/Update command in internal callbackMap, also adds/updates the callback of this command in the command cache.
     * <p>
     * This will also trigger a {@link #upsertCommand }.
     *
     * @param commandId command for which to replace the callback
     * @param callback  new callback to use
     */
    public static void upsertCallback(String commandId, TriggerCallback callback) {
        callbackMap.put(commandId, callback);
        var commandOptional = triggerService.getTriggersId(commandId);
        if (commandOptional.isEmpty()) {
            log.warn("Could not load command from Id, no command found with Id: {}", commandId);
            return;
        }
        upsertCommand(commandOptional.get());
    }

    /**
     * Updates the cache for this commandId
     * If this commands wasn't in the cache previously, it is added.
     * If the updated command is a callback command, the callback is preserved.
     */
    public static void upsertCommand(CommandEntity entity) {
        triggers.stream()
                .filter(trigger -> trigger.id().equals(entity.id))
                .findFirst()
                .ifPresent(triggers::remove);
        RunnableCommand runnable = commandEntityToRunnable(entity);
        if (runnable != null) {
            triggers.add(runnable);
        }
    }

    /**
     * Removes the command with this commandId from the command cache, and forget the callback, if one exists.
     * If the command isn't also removed from the database, it will likely later be reloaded as a text command
     *
     * @param commandId command which to forget
     */
    public static void unloadCommand(String commandId) {
        callbackMap.remove(commandId);
        triggers.stream()
                .filter(trigger -> trigger.id().equals(commandId))
                .findFirst()
                .ifPresent(triggers::remove);
    }
}
