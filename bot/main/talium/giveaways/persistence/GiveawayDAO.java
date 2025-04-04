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

    public UUID id() {
        return id;
    }

    public OffsetDateTime createdAt() {
        return createdAt;
    }

    public OffsetDateTime lastUpdatedAt() {
        return lastUpdatedAt;
    }

    public String title() {
        return title;
    }

    public String notes() {
        return notes;
    }

    public GiveawayStatus status() {
        return status;
    }

    public TriggerEntity command() {
        return command;
    }

    @Nullable
    public OffsetDateTime autoStart() {
        return autoStart;
    }

    @Nullable
    public OffsetDateTime autoEnd() {
        return autoEnd;
    }

    public int ticketCost() {
        return ticketCost;
    }

    public int maxTickets() {
        return maxTickets;
    }

    public boolean allowRedrawOfUser() {
        return allowRedrawOfUser;
    }

    public boolean autoAnnounceWinner() {
        return autoAnnounceWinner;
    }

    public List<EntriesDAO> ticketList() {
        return ticketList;
    }

    public List<WinnersDAO> winners() {
        return winners;
    }

    @Override
    public String toString() {
        return "GiveawayDAO{" +
                "id=" + id +
                ", createdAt=" + createdAt +
                ", lastUpdatedAt=" + lastUpdatedAt +
                ", title='" + title + '\'' +
                ", notes='" + notes + '\'' +
                ", status=" + status +
                ", command=" + command +
                ", autoStart=" + autoStart +
                ", autoEnd=" + autoEnd +
                ", ticketCost=" + ticketCost +
                ", maxTickets=" + maxTickets +
                ", allowRedrawOfUser=" + allowRedrawOfUser +
                ", autoAnnounceWinner=" + autoAnnounceWinner +
                ", ticketList=" + ticketList +
                ", winners=" + winners +
                '}';
    }
}


