package talium.system.inputSystem.configuration;

import talium.inputs.Twitch4J.TwitchUserPermission;
import talium.system.stringTemplates.Template;
import talium.system.twitchCommands.cooldown.ChatCooldown;
import talium.system.twitchCommands.cooldown.CooldownType;
import talium.system.twitchCommands.triggerEngine.RuntimeTrigger;
import talium.system.twitchCommands.triggerEngine.TriggerCallback;
import talium.system.twitchCommands.triggerEngine.TriggerEngine;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Pattern;

public record InputConfiguration(List<RuntimeTrigger> commands, List<Template> templates) {

    public static class Builder {
        private static final ChatCooldown DEFAULT_COOLDOWN = new ChatCooldown(CooldownType.MESSAGES, 0);
        List<RuntimeTrigger> commands = new ArrayList<>();
        List<Template> templates = new ArrayList<>();

        public Builder addCallbackCommand(RuntimeTrigger command) {
            commands.add(command);
            return this;
        }

        public Builder addCallbackCommand(String commandId, String pattern, TriggerCallback callback) {
            commands.add(new RuntimeTrigger(commandId, List.of(Pattern.compile(pattern)), TwitchUserPermission.EVERYONE, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, callback));
            return this;
        }

        public Builder addCallbackCommand(String commandId, List<String> pattern, TriggerCallback callback) {
            commands.add(new RuntimeTrigger(commandId, pattern.stream().map(Pattern::compile).toList(), TwitchUserPermission.EVERYONE, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, callback));
            return this;
        }

        public Builder addTextCommand(String commandId, String pattern, String template) {
            templates.add(new Template(commandId, template, null));
            commands.add(new RuntimeTrigger(commandId, List.of(Pattern.compile(pattern)), TwitchUserPermission.EVERYONE, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, TriggerEngine.TEXT_COMMAND_CALLBACK));
            return this;
        }

        public Builder addTextCommand(String commandId, List<String> pattern, String template) {
            templates.add(new Template(commandId, template, null));
            commands.add(new RuntimeTrigger(commandId, pattern.stream().map(Pattern::compile).toList(), TwitchUserPermission.EVERYONE, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, TriggerEngine.TEXT_COMMAND_CALLBACK));
            return this;
        }

        public Builder addTextCommand(String commandId, List<String> pattern, TwitchUserPermission permission, String template) {
            templates.add(new Template(commandId, template, null));
            commands.add(new RuntimeTrigger(commandId, pattern.stream().map(Pattern::compile).toList(), permission, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, TriggerEngine.TEXT_COMMAND_CALLBACK));
            return this;
        }

        public Builder addTextCommand(String commandId, List<String> pattern, TwitchUserPermission permission, String template, String messageColor) {
            templates.add(new Template(commandId, template, messageColor));
            commands.add(new RuntimeTrigger(commandId, pattern.stream().map(Pattern::compile).toList(), permission, DEFAULT_COOLDOWN, DEFAULT_COOLDOWN, TriggerEngine.TEXT_COMMAND_CALLBACK));
            return this;
        }


        public Builder addTemplate(Template template) {
            templates.add(template);
            return this;
        }

        public Builder addTemplate(String templateId, String template, String messageColor) {
            templates.add(new Template(templateId, template, messageColor));
            return this;
        }

        public Builder addTemplate(String templateId, String template) {
            templates.add(new Template(templateId, template, null));
            return this;
        }

        public InputConfiguration build() {
            return new InputConfiguration(commands, templates);
        }
    }
}
