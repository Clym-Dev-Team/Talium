package talium.giveaways.persistence;

import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

public interface WinnerRepo extends ListCrudRepository<WinnersDAO, EntriesDAO.EntriesId> {
    @Modifying
    @Transactional
    @Query("INSERT WinnersDAO (giveaway, userId, rejected, comment) VALUES (?1, ?2, false, null)")
    void addWinner(UUID giveawayId, String userId);

    @Query("SELECT WinnersDAO WHERE WinnersDAO.giveaway.id = ?1")
    List<WinnersDAO> getWinnersByGiveaway(UUID giveawayId);
}
