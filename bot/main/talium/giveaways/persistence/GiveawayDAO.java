package talium.giveaways.persistence;

import jakarta.persistence.*;
import org.springframework.lang.Nullable;
import talium.giveaways.GiveawayStatus;
import talium.twitchCommands.persistence.TriggerEntity;

import java.time.OffsetDateTime;
import java.util.*;

@Entity
@Table(name = "giveaways")
class GiveawayDAO {
    @Id
    long id;
    OffsetDateTime createdAt;
    OffsetDateTime lastUpdatedAt;
    String title;
    String notes;
    GiveawayStatus status;
    @OneToOne
    @JoinColumn(referencedColumnName = "id")
    TriggerEntity command;
    @Nullable OffsetDateTime autoStart;
    @Nullable OffsetDateTime autoEnd;
    int ticketCost;
    int maxTickets;
    boolean allowRedrawOfUser;
    boolean autoAnnounceWinner;
    @OneToMany(mappedBy = "giveaway", fetch = FetchType.EAGER)
    List<EntriesDAO> ticketList;
    @OneToMany(mappedBy = "giveaway", fetch = FetchType.EAGER)
    List<WinnersDAO> winners = new ArrayList<>();

    public GiveawayDAO() {
    }
}


