package talium.giveaways;

import jakarta.persistence.LockTimeoutException;
import org.apache.commons.lang3.math.NumberUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import talium.Out;
import talium.Registrar;
import talium.coinsWatchtime.chatter.ChatterRepo;
import talium.coinsWatchtime.chatter.ChatterService;
import talium.giveaways.persistence.*;
import talium.giveaways.transit.GiveawayDTO;
import talium.giveaways.transit.GiveawaySaveDTO;
import talium.twitch4J.TwitchUserPermission;
import talium.twitchCommands.cooldown.ChatCooldown;
import talium.twitchCommands.cooldown.CooldownType;
import talium.twitchCommands.persistence.CommandEntity;

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
    private final EntriesRepo entriesRepo;

    public GiveawayService(EntriesRepo entriesRepo) {
        this.entriesRepo = entriesRepo;
    }

    public static void init(GiveawayRepo giveawayRepo, ChatterService chatterService, ChatterRepo chatterRepo, TicketUpdater ticketUpdater) {
        GiveawayService.giveawayRepo = giveawayRepo;
        GiveawayService.chatterService = chatterService;
        GiveawayService.chatterRepo = chatterRepo;
        GiveawayService.ticketUpdater = ticketUpdater;
        Registrar.registerTemplate("giveaway.info", "@${sender} has ${senderCoins} Coins and ${senderTickets} Tickets. Usage: ${commandPattern} [amount]");
        Registrar.registerTemplate("giveaway.notOpen", "@${sender} the Giveaway ${commandPattern} is not yet open");
        Registrar.registerTemplate("giveaway.missingCoins", "@${sender} Not enough Coins for ${buyAmount} Tickets. You have ${senderCoins} Coins. ${giveaway.ticketCost} per Ticket");
        Registrar.registerTemplate("giveaway.retryableError", "@${sender} Failed to enter Giveaway. Please try again later");
        var activeGWs = giveawayRepo.findAllByStatusIsNot(GiveawayStatus.ARCHIVED);
        for (var gw : activeGWs) {
            createGWEnterCommand(gw.id(), gw.command().patterns.getFirst().pattern);
        }
    }

    private static final ReentrantReadWriteLock lock = new ReentrantReadWriteLock();

    /**
     * @param templateId ID of template to copy values from
     * @return UUID of created giveaway
     */
    public GiveawayDTO createFromTemplate(String templateId) {
        //TODO get giveaway template from DB
        GiveawayTemplateDAO template;
        if (templateId.equalsIgnoreCase("goldkette")) {
            template = GiveawayTemplateDAO.Goldkette();
        }  else if (templateId.equalsIgnoreCase("random")) {
            template = GiveawayTemplateDAO.Random();
        }
        else {
            template = GiveawayTemplateDAO.Blank();
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
                toSave.commandPattern(),
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
        // register autostart/close if necessary
        return giveawayRepo.save(giveaway);
    }

    public void updateGiveaway(GiveawaySaveDTO toUpdate, GiveawayDAO old) throws ChatterService.MissingDataException, InterruptedException {
        CommandEntity oldCommand = old.command();
        if (oldCommand != null) {
        if (!toUpdate.commandPattern().equals(oldCommand.patterns.getFirst().pattern)) {
            oldCommand.patterns.getFirst().pattern = toUpdate.commandPattern();
            Registrar.Command.upsert(oldCommand);
        }} else {
            createGWEnterCommand(old.id(), toUpdate.commandPattern());
        }
        var maxTicketsDecr = toUpdate.maxTickets() < old.maxTickets();
        var ticketCostChanged = toUpdate.ticketCost() != old.ticketCost();
        if (ticketCostChanged || maxTicketsDecr) {
            UUID gwId = old.id();
            try {
                if (lock.writeLock().tryLock(20, TimeUnit.SECONDS)) {
                    ticketUpdater.updateTicketsTransaction(gwId, toUpdate);
                } else {
                    logger.error("Failed to acquire lock to update giveaway, ID: {}, until timeout", gwId);
                    throw new LockTimeoutException("Failed to acquire lock to update giveaway, ID: " + gwId + ", until timeout");
                }
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                logger.error("Unexpectedly Interrupted while updating Giveaways. Interrupts are not part of the normal application flow here", e);
                throw e;
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
        // update autoOpen/Close
    }

    @Transactional
    public void refundAllTickets(GiveawayDAO giveaway) throws InterruptedException {
        try {
            if (lock.writeLock().tryLock(20, TimeUnit.SECONDS)) {
                var tickets = giveaway.ticketList();
                for (var ticket : tickets) {
                    entriesRepo.subtractTicketsByGiveawayId(giveaway.id(), ticket.tickets());
                    chatterRepo.addCoins(ticket.userId(), (long) ticket.tickets() * giveaway.ticketCost());
                }
            } else {
                logger.error("Failed to acquire lock to refund all tickets for giveaway, ID: {}, until timeout", giveaway.id());
                throw new LockTimeoutException("Failed to acquire lock to refund all tickets for giveaway, ID: " + giveaway.id() + ", until timeout");
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            logger.error("Unexpectedly Interrupted while refunding all tickets. Interrupts are not part of the normal application flow here", e);
            throw e;
        } finally {
            lock.writeLock().unlock();
        }
    }

    public void draw(GiveawayDAO giveaway) {
        //TODO
    }

    public void archive(GiveawayDAO giveaway) {
        String commandId = null;
        CommandEntity command = giveaway.command();
        if (command != null) {
            commandId = command.id;
        }
        giveaway.status(GiveawayStatus.ARCHIVED);
        giveaway.command(null);
        giveawayRepo.save(giveaway);
        if (commandId != null) {
            Registrar.Command.delete(commandId);
        }
    }


    public void unarchive(GiveawayDAO giveaway) {
        giveaway.command(createGWEnterCommand(giveaway.id(), giveaway.commandPattern()));
        giveaway.status(GiveawayStatus.PAUSED);
        giveawayRepo.save(giveaway);
    }

    public void deleteArchived(GiveawayDAO giveaway) {
        giveawayRepo.delete(giveaway);
    }

    private enum SubtractCoinsResult {
        SUCCESS,
        NOT_ENOUGH_COINS,
        INTERRUPTED,
        LOCK_TIMEOUT,
    }

    private static SubtractCoinsResult enterGwLocking(String userId, UUID gwId, int additionalCost, int ticketsToAdd) {
        try {
            if (lock.readLock().tryLock(2, TimeUnit.SECONDS)) {
                if (ticketUpdater.addTicketTransaction(gwId, userId, additionalCost, ticketsToAdd)) {
                    return SubtractCoinsResult.SUCCESS;
                }
                return SubtractCoinsResult.NOT_ENOUGH_COINS;
            } else {
                logger.error("Failed to acquire lock to subtract Coins for user {} until timeout", userId);
                return SubtractCoinsResult.LOCK_TIMEOUT;
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            logger.error("Unexpectedly Interrupted while subtracting coins. Interrupts are not part of the normal application flow here", e);
            return SubtractCoinsResult.INTERRUPTED;
        } finally {
            lock.readLock().unlock();
        }
    }

    private static CommandEntity createGWEnterCommand(UUID giveawayId, String commandPattern) {
        String commandId = STR."giveaway.\{giveawayId.toString()}.enter";
        return new Registrar
                .Command(commandId, commandPattern)
                .globalCooldown(new ChatCooldown(CooldownType.SECONDS, 0))
                .userCooldown(new ChatCooldown(CooldownType.SECONDS, 0))
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
                    values.put("senderTickets", gw
                            .get()
                            .ticketList()
                            .stream()
                            .filter(entriesDAO -> entriesDAO.userId().equals(message.user().id()))
                            .map(EntriesDAO::tickets)
                            .findFirst()
                            .orElse(0));
                    if (!hasAmount) {
                        Out.Twitch.sendNamedTemplate("giveaway.info", values);
                        return;
                    }

                    if (gw.get().status() != GiveawayStatus.RUNNING) {
                        // not yet open, this is an expected case, so log nothing, print nothing to the user
                        Out.Twitch.sendNamedTemplate("giveaway.notOpen", values);
                        return;
                    }
                    var ticketsToBuy = Integer.parseInt(split[1]);
                    var additionalCost = ticketsToBuy * gw.get().ticketCost();
                    var success = enterGwLocking(message.user().id(), giveawayId, additionalCost, ticketsToBuy);
                    switch (success) {
                        case SUCCESS -> { /* print nothing, this is the happy path */}
                        case NOT_ENOUGH_COINS -> {
                            values.put("buyAmount", ticketsToBuy);
                            Out.Twitch.sendNamedTemplate("giveaway.missingCoins", values);
                        }
                        // these are error cases, we decided against printing errors to the user
                        case INTERRUPTED -> {
                            logger.error("Acquiring lock to enter GW failed because of interrupt. For User: {}", message.user().name());
                            Out.Twitch.sendNamedTemplate("giveaway.retryableError", values);
                        }
                        case LOCK_TIMEOUT -> {
                            logger.error("Acquiring lock to enter GW failed because of lock timeout. For User: {}", message.user().name());
                            Out.Twitch.sendNamedTemplate("giveaway.retryableError", values);
                        }
                    }
                });
    }
}
