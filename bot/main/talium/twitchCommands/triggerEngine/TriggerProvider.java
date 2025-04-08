package talium.twitchCommands.triggerEngine;

import org.jetbrains.annotations.NotNull;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import talium.Registrar;
import talium.twitchCommands.persistence.MessagePattern;
import talium.twitchCommands.persistence.TriggerEntity;
import talium.twitchCommands.persistence.TriggerService;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Optional;
import java.util.regex.Pattern;

@Service
public class TriggerProvider {
    //TODO resetable cache

    /**
     * Cache of runtime triggers, should only be build from the db, and needs to be rebuilt on trigger change
     */
    private static List<RuntimeTrigger> triggers;
    private static TriggerService triggerService;
    private static final HashMap<String, RuntimeTrigger> codeTriggerMap = new HashMap<>();

    @Autowired
    public TriggerProvider(TriggerService triggerService) {
        TriggerProvider.triggerService = triggerService;
    }

    public static List<RuntimeTrigger> triggers() {
        return triggers;
    }


    /**
     * Rebuilds the cache of triggers
     */
    public static void rebuildTriggerCache() {
        triggers = buildTrigger();
    }

    /**
     * Builds the trigger list of triggers defined in code, and triggers form the trigger database.
     * If a particular triggerId is found in the code and the database the callback of the code trigger and
     * the data of the database trigger is used, since it may have been changed by the user.
     * @return a list of processed RuntimeTriggers
     */
    public static List<RuntimeTrigger> buildTrigger() {
        // get list of DB triggers (user commands, and system triggers from last starts)
        HashMap<String, TriggerEntity> dbCodeTriggers = triggerService.getCodeTriggers();

        HashMap<String, RuntimeTrigger> resultMap = triggerService.getUserTriggers();

        // get overlap between sets
        // if triggerId is on both sets, the values from the db are preferred, bot the callback is taken from the code
        // if an triggerId is only in the code not in the db, the full trigger from the code is used
        // if a trigger is only in the db and not in the code the triggerId is removed from the DB, because it is not used anymore
        for (var trigger : codeTriggerMap.values()) {
            if (dbCodeTriggers.containsKey(trigger.id())) {
                // triggerId is in both maps, use DB instance with code callback
                TriggerEntity triggerEntity = dbCodeTriggers.get(trigger.id());
                RuntimeTrigger runtimeTrigger = transformTrigger(triggerEntity, trigger.callback());
                resultMap.put(trigger.id(), runtimeTrigger);
                // remove triggerId from db callbacks so that we can later clean up all unused code callbacks from the DB
                dbCodeTriggers.remove(trigger.id());
            } else {
                // if triggerId is not in DB we use the code instance
                resultMap.put(trigger.id(), trigger);
            }
        }
        dbCodeTriggers.keySet().forEach(key -> triggerService.removeTrigger(key));

        return resultMap.values().stream().toList();
    }

    /**
     * Transforms a Chattrigger into a runtime Trigger for internal use of the triggerEngine.
     *
     * @apiNote Discards all disabled patterns
     * @param trigger  trigger to transform
     * @param callback callback to call on successfull trigger
     * @return transformed trigger
     */
    public static RuntimeTrigger transformTrigger(TriggerEntity trigger, TriggerCallback callback) {
        List<Pattern> regexes = new ArrayList<>();
        for (var pattern : trigger.patterns) {
            if (!pattern.isEnabled) {
                continue;
            }
            if (pattern.isRegex) {
                regexes.add(Pattern.compile(pattern.pattern));
            } else {
                regexes.add(compilePrefixPattern(pattern.pattern));
            }
        }
        return new RuntimeTrigger(
                trigger.id,
                regexes,
                trigger.permission,
                trigger.userCooldown,
                trigger.globalCooldown,
                callback
        );
    }

    private static @NotNull Pattern compilePrefixPattern(String prefixPattern) {
        return Pattern.compile(STR."^\{prefixPattern} ?.*", Pattern.CASE_INSENSITIVE);
    }

    /**
     * Add the command to an internal list to use while building the final cache of commands.
     *
     * @param command             command
     * @param textCommandCallback
     */
    public static TriggerEntity addCommandRegistration(Registrar.Command command, TriggerCallback callback) {
        var messagePatterns = new ArrayList<>(command.regexPattern().stream().map(pattern -> new MessagePattern(pattern.pattern(), true, false, true)).toList());
        messagePatterns.addAll(command.prefixPattern().stream().map(pattern -> new MessagePattern(pattern, false, false, true)).toList());


        command.regexPattern().addAll(command.prefixPattern().stream().map(TriggerProvider::compilePrefixPattern).toList());
        var trigger = new RuntimeTrigger(command.id(), command.regexPattern(), command.permission(), command.userCooldown(),command.globalCooldown(), callback);
        codeTriggerMap.put(command.id(), trigger);
        Optional<TriggerEntity> dbTrigger = triggerService.getTriggersId(command.id());
        if (dbTrigger.isPresent()) {
            return dbTrigger.get();
        }


        TriggerEntity entity = new TriggerEntity(command.id(), "", messagePatterns, command.permission(), command.userCooldown(), command.globalCooldown(), true, null);
        try {
            return triggerService.save(entity);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null;
    }
}
