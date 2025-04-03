package talium.giveaways.persistence;

import jakarta.persistence.*;
import org.springframework.lang.Nullable;
import talium.giveaways.GiveawayStatus;
import talium.twitchCommands.persistence.TriggerEntity;

import java.time.OffsetDateTime;
import java.util.*;

@Entity
@Table(name = "giveaways")
public class GiveawayDAO {
    @Id
    UUID id;
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
    List<EntriesDAO> ticketList = new ArrayList<>();
    @OneToMany(mappedBy = "giveaway", fetch = FetchType.EAGER)
    List<WinnersDAO> winners = new ArrayList<>();

    public GiveawayDAO() {
    }

    public GiveawayDAO(UUID id, OffsetDateTime createdAt, OffsetDateTime lastUpdatedAt, String title, String notes, GiveawayStatus status, TriggerEntity command, @Nullable OffsetDateTime autoStart, @Nullable OffsetDateTime autoEnd, int ticketCost, int maxTickets, boolean allowRedrawOfUser, boolean autoAnnounceWinner, List<EntriesDAO> ticketList, List<WinnersDAO> winners) {
        this.id = id;
        this.createdAt = createdAt;
        this.lastUpdatedAt = lastUpdatedAt;
        this.title = title;
        this.notes = notes;
        this.status = status;
        this.command = command;
        this.autoStart = autoStart;
        this.autoEnd = autoEnd;
        this.ticketCost = ticketCost;
        this.maxTickets = maxTickets;
        this.allowRedrawOfUser = allowRedrawOfUser;
        this.autoAnnounceWinner = autoAnnounceWinner;
        this.ticketList = ticketList;
        this.winners = winners;
    }

    public GiveawayDAO(String title, String notes, TriggerEntity command, @Nullable OffsetDateTime autoStart, @Nullable OffsetDateTime autoEnd, int ticketCost, int maxTickets, boolean allowRedrawOfUser, boolean autoAnnounceWinner) {
        this.id = UUID.randomUUID();
        this.createdAt = OffsetDateTime.now();
        this.lastUpdatedAt = OffsetDateTime.now();
        this.title = title;
        this.notes = notes;
        this.status = GiveawayStatus.CREATED;
        this.command = command;
        this.autoStart = autoStart;
        this.autoEnd = autoEnd;
        this.ticketCost = ticketCost;
        this.maxTickets = maxTickets;
        this.allowRedrawOfUser = allowRedrawOfUser;
        this.autoAnnounceWinner = autoAnnounceWinner;
    }
}


