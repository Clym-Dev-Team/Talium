package talium.twitchCommands.controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.lang.Nullable;
import org.springframework.web.bind.annotation.*;
import talium.Registrar;
import talium.twitchCommands.persistence.CommandEntity;
import talium.twitchCommands.persistence.TriggerService;

import static talium.TwitchBot.gson;

@RestController
@RequestMapping(value = "/commands", produces = "application/json")
public class CommandController {
    TriggerService triggerService;

    @Autowired
    public CommandController(TriggerService triggerService) {
        this.triggerService = triggerService;
    }

    @GetMapping("/userAll")
    String getAllUserCommands(@RequestParam @Nullable String search) {
        if (search == null || search.isEmpty()) {
            var list = triggerService.getAllUserTriggers().stream().map(CommandEntity::toTriggerDTO).toList();
            return gson.toJson(list);
        }
        var list = triggerService.searchUserBy(search).stream().map(CommandEntity::toTriggerDTO).toList();
        return gson.toJson(list);
    }

    @GetMapping("/all")
    String getAllCommands(@RequestParam @Nullable String search) {
        if (search == null || search.isEmpty()) {
            var list = triggerService.getAllTriggers().stream().map(CommandEntity::toTriggerDTO).toList();
            return gson.toJson(list);
        }
        var list = triggerService.searchBy(search).stream().map(CommandEntity::toTriggerDTO).toList();
        return gson.toJson(list);
    }

    @GetMapping("/id/{triggerId}")
    ResponseEntity<String> getByTriggerId(@PathVariable String triggerId) {
        var command = triggerService.getTriggersId(triggerId);
        if (command.isEmpty()) {
            return ResponseEntity.notFound().build();
        }
        return ResponseEntity.ok(gson.toJson(command.get().toTriggerDTO(), TriggerDTO.class));
    }

    @PostMapping("/id/{triggerId}/set/enabled")
    HttpStatus setEnabled(@PathVariable String triggerId, @RequestBody String enabled) {
        var trigger = triggerService.getTriggersId(triggerId);
        if (trigger.isEmpty()) {
            return HttpStatus.NOT_FOUND;
        }
        triggerService.setEnabled(trigger.get(), Boolean.parseBoolean(enabled));
        return HttpStatus.OK;
    }

    @PostMapping("/id/{triggerId}/set/visible")
    HttpStatus setVisible(@PathVariable String triggerId, @RequestBody String visible) {
        var trigger = triggerService.getTriggersId(triggerId);
        if (trigger.isEmpty()) {
            return HttpStatus.NOT_FOUND;
        }
        triggerService.setVisible(trigger.get(), Boolean.parseBoolean(visible));
        return HttpStatus.OK;
    }

    @PostMapping("/save")
    HttpStatus saveCommand(@RequestBody String body) {
        TriggerDTO command = gson.fromJson(body, TriggerDTO.class);
        Registrar.Command.upsert(new CommandEntity(command));
        return HttpStatus.OK;
    }

    @DeleteMapping("/delete/{id}")
    HttpStatus deleteCommand(@PathVariable String id) {
        if (triggerService.getTriggersId(id).isEmpty()) {
            return HttpStatus.NOT_FOUND;
        }
        triggerService.delete(id);
        return HttpStatus.OK;
    }
}
