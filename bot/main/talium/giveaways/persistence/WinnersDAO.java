package talium.giveaways.persistence;

import jakarta.persistence.*;
import org.springframework.lang.Nullable;

@Entity
@Table(name = "giveawayWinners")
@IdClass(EntriesDAO.EntriesId.class)
public class WinnersDAO {
    @Id @ManyToOne(fetch = FetchType.EAGER)
    GiveawayDAO giveaway;
    @Id
    String userId;
    boolean rejected;
    @Nullable String comment;

    protected WinnersDAO() {}

    public String userId() {
        return userId;
    }

    public boolean rejected() {
        return rejected;
    }

    @Nullable
    public String comment() {
        return comment;
    }
}
