package talium.giveaways.transit;

import java.time.Instant;
import java.util.Optional;

/// Used to display Giveaways
public record GiveawaySaveDTO(
    String title,
    String notes,
    String commandId,     // TODO questionable if this is needed/appropriate here. We could not rely on this to save,
                          //  because this should be managed by the bot, not the user. And it would allow you to overwrite unrelated commands,
                          //  so this would need to be fetched from the DB. Also this is not needed to display the command, so it is unnecessary there too
    String commandFirstPattern,
    String commandTemplate,
    Optional<Instant> autoStart,
    Optional<Instant> autoEnd,
    int ticketCost,
    int maxTickets,
    boolean allowRedrawOfUser,
    boolean autoAnnounceWinner
) {}
