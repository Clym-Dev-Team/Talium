package talium.giveaways.persistence;

import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

public interface EntriesRepo extends ListCrudRepository<EntriesDAO, EntriesDAO.EntriesId> {
    @Modifying
    @Transactional
    @Query("UPDATE EntriesDAO SET tickets = tickets - ?2 WHERE giveaway.id = ?1")
    void subtractTicketsByGiveawayId(UUID giveawayId, int tickets);
}
