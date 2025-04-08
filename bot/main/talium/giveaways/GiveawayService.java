package talium.giveaways;

import org.apache.commons.lang3.math.NumberUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import talium.Out;
import talium.Registrar;
import talium.coinsWatchtime.chatter.ChatterRepo;
import talium.coinsWatchtime.chatter.ChatterService;
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
import java.util.HashMap;
import java.util.UUID;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.ReentrantReadWriteLock;

@Service
public class GiveawayService {
    private static final Logger logger = LoggerFactory.getLogger(GiveawayService.class);
    private static GiveawayRepo giveawayRepo;
    private static ChatterService chatterService;
    private static ChatterRepo chatterRepo;
    private static TicketUpdater ticketUpdater;

    public static void init(GiveawayRepo giveawayRepo, ChatterService chatterService, ChatterRepo chatterRepo, TicketUpdater ticketUpdater) {
        GiveawayService.giveawayRepo = giveawayRepo;
        GiveawayService.chatterService = chatterService;
        GiveawayService.chatterRepo = chatterRepo;
        GiveawayService.ticketUpdater = ticketUpdater;
        Registrar.registerTemplate("giveaway.info", "@${sender} has ${senderCoins} Coins. Usage: ${commandPattern} [amount]");
        Registrar.registerTemplate("giveaway.missingCoins", "@${sender} Not enough Coins for ${buyAmount} Tickets. You have ${senderCoins} Coins. ${giveaway.ticketCost} per Ticket");
        var activeGWs = giveawayRepo.findAllByStatusIsNot(GiveawayStatus.ARCHIVED);
        for (var gw : activeGWs) {
            createGWEnterCommand(gw.id(), gw.command().id);
        }
    }

    private static ReentrantReadWriteLock lock = new ReentrantReadWriteLock();

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

    public void updateGiveaway(GiveawaySaveDTO toUpdate, GiveawayDAO old) throws ChatterService.MissingDataException {
        if (!toUpdate.commandPattern().equals(old.command().patterns.getFirst().pattern)) {
            //TODO update command for GW
        }
        var maxTicketsDecr = toUpdate.maxTickets() < old.maxTickets();
        var ticketCostChanged = toUpdate.ticketCost() != old.ticketCost();
        if (ticketCostChanged || maxTicketsDecr) {
            UUID gwId = old.id();
            lock.writeLock().lock();
            try {
                ticketUpdater.updateTicketsTransaction(gwId, toUpdate);
            } finally {
                lock.writeLock().unlock();
            }
        } else {
            giveawayRepo.update(
                    old.id(),
                    toUpdate.title(),
                    toUpdate.notes(),
                    toUpdate.autoOpen(),
                    toUpdate.autoClose(),
                    toUpdate.ticketCost(),
                    toUpdate.maxTickets(),
                    toUpdate.allowRedrawOfUser(),
                    toUpdate.autoAnnounceWinner()
            );
        }
        //TODO update autoOpen/Close
    }


    private enum SubtractCoinsResult {
        SUCCESS,
        NOT_ENOUGH_COINS,
        INTERRUPTED,
        LOCK_TIMEOUT,
    }

    private static SubtractCoinsResult subtractCoinsLocking(String userId, int coins) {
        try {
            if (lock.readLock().tryLock(2, TimeUnit.SECONDS)) {
                try {
                    if (chatterRepo.addCoins(userId, -coins) == 1) {
                        return SubtractCoinsResult.SUCCESS;
                    }
                    return SubtractCoinsResult.NOT_ENOUGH_COINS;
                } finally {
                    lock.readLock().unlock();
                }
            } else {
                return SubtractCoinsResult.LOCK_TIMEOUT;
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            logger.error("Unexpectedly Interrupted while subtracting coins. Interrupts are not part of the normal application flow here", e);
            return SubtractCoinsResult.INTERRUPTED;
        }
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
                    var text = message.message();
                    var split = text.split(" ");
                    var hasAmount = split.length > 1 && NumberUtils.isDigits(split[1]);

                    var gw = giveawayRepo.findById(giveawayId);
                    if (gw.isEmpty()) {
                        logger.warn("Tried to enter a non existing Giveaway, ID: {}. Failed to unregister Enter command?", giveawayId);
                        // gw doesn't exist (anymore), print nothing to the user
                        return;
                    }

                    var values = new HashMap<String, Object>();
                    values.put("sender", message.user().name());
                    values.put("giveaway", gw.get());
                    values.put("commandPattern", gw.get().command().patterns.getFirst().pattern);
                    values.put("senderCoins", chatterService.getChatterDataOrDefault(message.user().id()).coins);
                    if (!hasAmount) {
                        Out.Twitch.sendNamedTemplate("giveaway.info", values);
                        return;
                    }

                    if (gw.get().status() != GiveawayStatus.RUNNING) {
                        // not yet open, this is an expected case, so log nothing, print nothing to the user
                        return;
                    }
                    var ticketsToBuy = Integer.parseInt(split[1]);
                    var additionalCost = ticketsToBuy * gw.get().ticketCost();
                    var success = subtractCoinsLocking(message.user().id(), additionalCost);
                    switch (success) {
                        case SUCCESS -> { /* print nothing, this is the happy path */}
                        case NOT_ENOUGH_COINS -> {
                            values.put("buyAmount", ticketsToBuy);
                            Out.Twitch.sendNamedTemplate("giveaway.missingCoins", values);
                        }
                        // these are error cases, we decided against printing errors to the user
                        case INTERRUPTED ->
                                logger.error("Acquiring lock to enter GW failed because of interrupt. For User: {}", message.user().name());
                        case LOCK_TIMEOUT ->
                                logger.error("Acquiring lock to enter GW failed because of lock timeout. For User: {}", message.user().name());
                    }
                });
    }
}
