package talium.giveaways.persistence;

import jakarta.persistence.*;
import org.jetbrains.annotations.NotNull;
import org.springframework.lang.Nullable;
import talium.giveaways.GiveawayStatus;
import talium.twitchCommands.persistence.CommandEntity;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

@Entity
@Table(name = "giveaways")
public class GiveawayDAO {
    @Id
    UUID id;
    Instant createdAt;
    Instant lastUpdatedAt;
    String title;
    String notes;
    GiveawayStatus status;
    @OneToOne
    @JoinColumn(referencedColumnName = "id")
    @Nullable
    CommandEntity command;
    @Nullable Instant autoStart;
    @Nullable Instant autoEnd;
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

    public GiveawayDAO(UUID id, Instant createdAt, Instant lastUpdatedAt, String title, String notes, GiveawayStatus status, @NotNull CommandEntity command, @Nullable Instant autoStart, @Nullable Instant autoEnd, int ticketCost, int maxTickets, boolean allowRedrawOfUser, boolean autoAnnounceWinner, List<EntriesDAO> ticketList, List<WinnersDAO> winners) {
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

    public Instant createdAt() {
        return createdAt;
    }

    public Instant lastUpdatedAt() {
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

    @Nullable
    public CommandEntity command() {
        return command;
    }

    @Nullable
    public Instant autoStart() {
        return autoStart;
    }

    @Nullable
    public Instant autoEnd() {
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

    public GiveawayDAO command(@Nullable CommandEntity command) {
        this.command = command;
        return this;
    }

    public GiveawayDAO status(GiveawayStatus status) {
        this.status = status;
        return this;
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
                ", autoOpen=" + autoStart +
                ", autoClose=" + autoEnd +
                ", ticketCost=" + ticketCost +
                ", maxTickets=" + maxTickets +
                ", allowRedrawOfUser=" + allowRedrawOfUser +
                ", autoAnnounceWinner=" + autoAnnounceWinner +
                ", ticketList=" + ticketList +
                ", winners=" + winners +
                '}';
    }
}


