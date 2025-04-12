package talium.twitchCommands.triggerEngine;

import talium.twitch4J.TwitchUserPermission;
import talium.twitchCommands.cooldown.ChatCooldown;
import talium.twitchCommands.persistence.CommandEntity;

import java.util.List;
import java.util.regex.Pattern;

/**
 * A version of the {@link CommandEntity} modified for runtime use in the {@link TriggerEngine}.
 * The Patterns are unified in a list of Regex Pattern. Callback functions an added, these are executed if a message satisfies all these conditions.
 *
 * @param id             a unique triggerId that identifies this trigger. Is not allowed to collide with other triggerIds
 * @param patterns       a list of regex Patterns that the message content is matched against. It is enough if any of these patterns match
 * @param permission     a minimum permission level a user of the message needs to have
 * @param userCooldown   a user specific cooldown for this trigger
 * @param globalCooldown a global (for all users) cooldown for this trigger
 * @param callback       the function to call after a successfull trigger
 */
public record RunnableCommand(
        String id,
        List<Pattern> patterns,
        TwitchUserPermission permission,
        ChatCooldown userCooldown,
        ChatCooldown globalCooldown,
        TriggerCallback callback
) {
}
