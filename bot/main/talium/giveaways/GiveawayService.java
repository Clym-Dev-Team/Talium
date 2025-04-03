package talium.giveaways;

import org.springframework.stereotype.Service;
import talium.Registrar;
import talium.giveaways.persistence.GiveawayDAO;
import talium.giveaways.persistence.GiveawayRepo;
import talium.giveaways.persistence.GiveawayTemplateDAO;
import talium.twitch4J.TwitchUserPermission;
import talium.twitchCommands.cooldown.ChatCooldown;
import talium.twitchCommands.cooldown.CooldownType;
import talium.twitchCommands.persistence.TriggerEntity;
import talium.twitchCommands.triggerEngine.TriggerProvider;

import java.time.OffsetDateTime;
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
    public UUID createFromTemplate(String templateId) {
        //TODO get giveaway template from DB
        var template = new GiveawayTemplateDAO();
        UUID giveawayId = UUID.randomUUID();
        TriggerEntity commandEntity = createGWEnterCommand(giveawayId, template.commandPattern);
        var giveaway = new GiveawayDAO(
                giveawayId,
                OffsetDateTime.now(),
                OffsetDateTime.now(),
                template.title,
                template.notes,
                GiveawayStatus.CREATED,
                commandEntity,
                null,
                null,
                template.ticketCost,
                template.maxTickets,
                template.allowRedrawOfUser,
                template.autoAnnounceWinner,
                new ArrayList<>(),
                new ArrayList<>()
        );
        var savedId = giveawayRepo.save(giveaway).id();
        TriggerProvider.rebuildTriggerCache();
        return savedId;
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
