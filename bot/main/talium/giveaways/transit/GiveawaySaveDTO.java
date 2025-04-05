package talium.giveaways.transit;


import java.time.Instant;
import java.util.Optional;

/// Used to display Giveaways
public record GiveawaySaveDTO(
    String title,
    String notes,
    String commandFirstPattern,
    Optional<Instant> autoStart,
    Optional<Instant> autoEnd,
    int ticketCost,
    int maxTickets,
    boolean allowRedrawOfUser,
    boolean autoAnnounceWinner
) {}
