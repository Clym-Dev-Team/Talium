package talium.twitchCommands.triggerEngine;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Component;
import talium.Out;
import talium.eventSystem.Subscriber;
import talium.stringTemplates.TemplateService;
import talium.twitch4J.ChatMessage;

import static talium.twitchCommands.cooldown.CooldownService.*;
import static talium.twitchCommands.triggerEngine.CommandWithCallbackCache.triggers;

//TODO reference triggerId guidelines

/**
 * Evaluates if a message satisfies all the conditions of a trigger.
 * If a message matches, a callback function is called with this message.
 */
@Component
public class TriggerEngine {

    public static final TriggerCallback TEXT_COMMAND_CALLBACK = TriggerEngine::executeTextCommand;
    private static TemplateService templateService;
    private static final Logger logger = LoggerFactory.getLogger(TriggerEngine.class);

    @Autowired
    public TriggerEngine(TemplateService templateService) {
        TriggerEngine.templateService = templateService;
    }

    /**
     * Consumes {@link ChatMessage}s from the Twitch Input and checks if any triggers match this message.
     * If so, they callbacks are executed.
     *
     * @param messsage the message to check
     */
    @Subscriber
    public static void triggerFromMessage(ChatMessage messsage) {
        triggers().forEach(t -> processTrigger(t, messsage));
    }

    /**
     * Check if a message matches a specific trigger, and call this trigger
     *
     * @param trigger the trigger to check for
     * @param message the message to check against
     */
    private static void processTrigger(RunnableCommand trigger, ChatMessage message) {
        if (trigger == null) {
            logger.error("Trigger is null! Skipping trigger");
            return;
        }
        if (message == null) {
            logger.error("Message is null! Skipping trigger");
            return;
        }

        // if ordinal of user is smaller than the command/trigger, than the user has fewer permissions and is not allowed to execute
        if (message.user().permission().ordinal() < trigger.permission().ordinal()) {
            logger.debug("User {} with {}, missing {} permission for command {}", message.user().name(), message.user().permission(), trigger.permission(), trigger.id());
            return;
        }

        boolean isMatched = false;
        for (var pat : trigger.patterns()) {
            if (pat.matcher(message.message()).matches()) {
                isMatched = true;
                break;
            }
        }
        if (!isMatched) {
            return;
        }

        if (inGlobalCooldown(message, trigger.id(), trigger.globalCooldown())) {
            logger.debug("Call to command {} from {} rejected because of global cooldowns", trigger.id(), message.user().name());
            return;
        }
        if (inUserCooldown(message, trigger.id(), trigger.userCooldown())) {
            logger.debug("Call to command {} from {} rejected because of user cooldowns", trigger.id(), message.user().name());
            return;
        }
        updateCooldownState(message, trigger.id(), trigger.globalCooldown(), trigger.userCooldown());

        try {
            trigger.callback().triggerCallback(trigger.id(), message);
        } catch (Exception e) {
            logger.error("Error thrown in callback of command {}", trigger.id(), e);
        }
    }

    public static void executeTextCommand(String commandId, ChatMessage message) {
        logger.debug("Executing text command {}", commandId);
        var template = templateService.getTemplateByCommandId(commandId);
        if (template.isEmpty()) {
            logger.error("Could not find template id for command id {}", commandId);
            return;
        }
        //TODO add message and other things to context, but currently we can't handle records
        Out.Twitch.sendRawTemplate(template.get().template, null);
    }

}
