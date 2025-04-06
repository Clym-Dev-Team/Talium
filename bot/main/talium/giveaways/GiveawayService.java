package talium.giveaways;

import org.apache.commons.lang.NotImplementedException;
import org.springframework.stereotype.Service;
import talium.Registrar;
import talium.giveaways.persistence.GiveawayDAO;
import talium.giveaways.persistence.GiveawayRepo;
import talium.giveaways.persistence.GiveawayTemplateDAO;
import talium.giveaways.transit.GiveawayDTO;
import talium.giveaways.transit.GiveawaySaveDTO;
import talium.twitch4J.TwitchUserPermission;
import talium.twitchCommands.cooldown.ChatCooldown;
import talium.twitchCommands.cooldown.CooldownType;
import talium.twitchCommands.persistence.TriggerEntity;
import talium.twitchCommands.triggerEngine.TriggerProvider;

import java.time.Instant;
import java.util.ArrayList;
import java.util.UUID;

@Service
public class GiveawayService {
    private static GiveawayRepo giveawayRepo;

    public static void init(GiveawayRepo giveawayRepo) {
        GiveawayService.giveawayRepo = giveawayRepo;
        var activeGWs = giveawayRepo.findAllByStatusIsNot(GiveawayStatus.ARCHIVED);
        for (var gw : activeGWs) {
            createGWEnterCommand(gw.id(), gw.command().id);
        }
    }

    /**
     * @param templateId ID of template to copy values from
     * @return UUID of created giveaway
     */
    public GiveawayDTO createFromTemplate(String templateId) {
        //TODO get giveaway template from DB
        GiveawayTemplateDAO template;
        if (templateId.equals("goldkette")) {
            template = GiveawayTemplateDAO.Goldkette();
        } else {
            template = GiveawayTemplateDAO.Random();
        }
        UUID giveawayId = UUID.randomUUID();
        return new GiveawayDTO(
                giveawayId,
                template.title,
                template.notes,
                Instant.now().toString(),
                Instant.now().toString(),
                GiveawayStatus.PAUSED,
                template.commandPattern,
                null,
                null,
                template.ticketCost,
                template.maxTickets,
                template.allowRedrawOfUser,
                template.autoAnnounceWinner,
                new ArrayList<>(),
                new ArrayList<>()
        );
    }

    public GiveawayDAO saveNewGiveaway(GiveawaySaveDTO toSave) {
        UUID giveawayId = UUID.randomUUID();
        var command = createGWEnterCommand(giveawayId, toSave.commandPattern());
        var giveaway = new GiveawayDAO(
                giveawayId,
                Instant.now(),
                Instant.now(),
                toSave.title(),
                toSave.notes(),
                GiveawayStatus.PAUSED,
                command,
                toSave.autoOpen(),
                toSave.autoClose(),
                toSave.ticketCost(),
                toSave.maxTickets(),
                toSave.allowRedrawOfUser(),
                toSave.autoAnnounceWinner(),
                new ArrayList<>(),
                new ArrayList<>()
        );
        TriggerProvider.rebuildTriggerCache();
        //TODO register autostart/close if necessary
        return giveawayRepo.save(giveaway);
    }

    public GiveawayDAO updateGiveaway(GiveawaySaveDTO toUpdate) {
        //TODO save all to DB
        //TODO update command for GW
        //TODO trigger possible correction of amount and ticket amounts
        // for this i need a primitive lock
        // set lock, then change ticket cost in DB
        // adjust all previously bought tickets
        // release lock
        // all enter commands would check for lock, if lock set, enter sleep loop
        // until lock no longer set, if enter command is in sleep because of lock for more than 1 second
        // then cancel entry request and emit error
        // lock can be global for all GWs and only local in memory, this operation will not be performed very often
        //TODO trigger update of autostart/close timers
        throw new NotImplementedException();
    }

    private static TriggerEntity createGWEnterCommand(UUID giveawayId, String commandPattern) {
        String commandId = STR."giveaway.\{giveawayId.toString()}.enter";
        return new Registrar
                .Command(commandId)
                .globalCooldown(new ChatCooldown(CooldownType.SECONDS, 0))
                .userCooldown(new ChatCooldown(CooldownType.SECONDS, 0))
                .pattern(commandPattern)
                .permission(TwitchUserPermission.EVERYONE)
                .registerActionCommand((triggerId, message) -> {
                    //TODO do enter in giveaway
                    System.out.println("ENTERING GW: " + giveawayId + " for user: " + message.user().name());
                });
    }
}
