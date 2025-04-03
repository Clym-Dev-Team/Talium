package talium.giveaways.transit;


import talium.giveaways.GiveawayStatus;
import talium.giveaways.persistence.GiveawayDAO;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

/// Used to display Giveaways
public record GiveawayDTO(
        UUID id,
        String title,
        String notes,
        OffsetDateTime createdAt,
        OffsetDateTime lastUpdatedAt,
        GiveawayStatus status,
        String commandPattern,
        Optional<OffsetDateTime> autoStart,
        Optional<OffsetDateTime> autoEnd,
        int ticketCost,
        int maxTickets,
        boolean allowRedrawOfUser,
        boolean autoAnnounceWinner,
        List<EntryDTO> ticketList,
        List<WinnerDTO> winners
) {}
