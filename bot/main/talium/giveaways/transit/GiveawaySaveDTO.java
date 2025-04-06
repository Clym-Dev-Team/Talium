package talium.giveaways.transit;


import org.springframework.lang.Nullable;

import java.time.Instant;

/// Used to display Giveaways
public record GiveawaySaveDTO(
    String title,
    String notes,
    String commandPattern,
    @Nullable Instant autoOpen,
    @Nullable Instant autoClose,
    int ticketCost,
    int maxTickets,
    boolean allowRedrawOfUser,
    boolean autoAnnounceWinner
) {}
