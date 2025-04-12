package talium.giveaways;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import talium.coinsWatchtime.chatter.ChatterRepo;
import talium.coinsWatchtime.chatter.ChatterService;
import talium.giveaways.persistence.EntriesDAO;
import talium.giveaways.persistence.EntriesRepo;
import talium.giveaways.persistence.GiveawayRepo;
import talium.giveaways.transit.GiveawaySaveDTO;

import java.util.UUID;

@Service
public class TicketUpdater {
    private final ChatterService chatterService;
    private final EntriesRepo entriesRepo;
    GiveawayRepo giveawayRepo;
    ChatterRepo chatterRepo;

    @Autowired
    public TicketUpdater(GiveawayRepo giveawayRepo, ChatterRepo chatterRepo, ChatterService chatterService, EntriesRepo entriesRepo) {
        this.giveawayRepo = giveawayRepo;
        this.chatterRepo = chatterRepo;
        this.chatterService = chatterService;
        this.entriesRepo = entriesRepo;
    }

    @Transactional
    protected void updateTicketsTransaction(UUID gwId, GiveawaySaveDTO toUpdate) throws ChatterService.MissingDataException {
        // request new after lock, because only after getting the lock can we be sure that no changes can be made
        var old = giveawayRepo.findById(gwId);
        if (old.isEmpty()) {
            throw new RuntimeException("Could not find giveaway with id " + gwId);
        }
        var oldTicketCost = old.get().ticketCost();
        var newCost = toUpdate.ticketCost();
        var newMax = toUpdate.maxTickets();

        for (var ticket : old.get().ticketList()) {
            var userCoins = chatterService.getChatterDataSecure(ticket.userId()).coins;

            var totalInvested = ticket.tickets() * oldTicketCost;
            // rounding down to int is fine
            var couldBuy = (userCoins + totalInvested) / newCost;
            var newTickets = Math.min(Math.min(ticket.tickets(), couldBuy), newMax);
            var missingCoins = newTickets * newCost - totalInvested;
            // apply delta to coins
            if (missingCoins != 0) {
                chatterRepo.addCoins(ticket.userId(), -missingCoins);
            }
            if (newTickets != oldTicketCost) {
                entriesRepo.save(new EntriesDAO(old.get(), ticket.userId(), (int) newTickets));
            }
        }
        giveawayRepo.update(
                old.get().id(),
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

    @Transactional
    protected boolean addTicketTransaction(UUID gwId, String userId, int additionalCost, int ticketsToAdd) {
        if (chatterRepo.addCoins(userId, -additionalCost) == 0) {
            return false;
        }
//        var entriesRepo.existsById(gwId, userId);v
        //TODO either update old value, or add new entry
        return true;
    }

}
