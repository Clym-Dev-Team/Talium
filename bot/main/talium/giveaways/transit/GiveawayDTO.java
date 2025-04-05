package talium.giveaways.transit;


import org.springframework.lang.Nullable;
import talium.giveaways.GiveawayStatus;

import java.util.List;
import java.util.UUID;

/// Used to display Giveaways
public record GiveawayDTO(
        UUID id,
        String title,
        String notes,
        String createdAt,
        String lastUpdatedAt,
        GiveawayStatus status,
        String commandPattern,
        @Nullable String autoStart,
        @Nullable String autoEnd,
        int ticketCost,
        int maxTickets,
        boolean allowRedrawOfUser,
        boolean autoAnnounceWinner,
        List<EntryDTO> ticketList,
        List<WinnerDTO> winners
) {}
