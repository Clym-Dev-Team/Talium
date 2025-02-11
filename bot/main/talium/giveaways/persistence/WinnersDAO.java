package talium.giveaways.persistence;

import jakarta.persistence.*;
import org.springframework.lang.Nullable;

@Entity
@Table(name = "giveawayWinners")
@IdClass(EntriesDAO.EntriesId.class)
class WinnersDAO {
    @Id @ManyToOne(fetch = FetchType.EAGER)
    GiveawayDAO giveaway;
    @Id
    String userId;
    boolean rejected;
    @Nullable String comment;

    protected WinnersDAO() {}

    protected WinnersDAO(GiveawayDAO giveaway, String userId, boolean rejected, @Nullable String comment) {
        this.giveaway = giveaway;
        this.userId = userId;
        this.rejected = rejected;
        this.comment = comment;
    }
}
