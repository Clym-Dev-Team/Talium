package talium.giveaways.persistence;

import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.transaction.annotation.Transactional;

import java.util.Optional;
import java.util.UUID;

public interface EntriesRepo extends ListCrudRepository<EntriesDAO, EntriesDAO.EntriesId> {
    @Modifying
    @Transactional
    @Query("UPDATE EntriesDAO SET tickets = MAX(tickets - ?2, 0) WHERE giveaway.id = ?1")
    void subtractTicketsByGiveawayId(UUID giveawayId, int tickets);

    @Modifying
    @Transactional
    @Query("UPDATE EntriesDAO SET tickets = MAX(tickets - ?3, 0) WHERE giveaway.id = ?1 AND userId = ?2")
    void subtractTicketForEntry(UUID giveawayId, String userId, int tickets);

    Optional<Integer> getTicketsByGiveawayIdAndUserId(UUID giveaway_id, String userId);

    @Modifying
    @Transactional
    @Query("INSERT EntriesDAO (giveaway, userId, tickets) VALUES (?1, ?2, ?3)")
    void createEntry(UUID giveawayId, String userId, int tickets);


    @Modifying
    @Transactional
    @Query("UPDATE EntriesDAO SET tickets = tickets + ?3 WHERE giveaway.id = ?1 AND userId = ?2")
    void addTicketsForUser(UUID giveawayId, String userId, int tickets);

    @Modifying
    @Transactional
    @Query("DELETE EntriesDAO WHERE giveaway.id = ?1 AND userId = ?2")
    void removeEntry(UUID giveawayId, String userId);

    @Modifying
    @Transactional
    @Query("DELETE EntriesDAO WHERE giveaway.id = ?1 AND tickets = 0")
    void removeEntriesWithZeroTicketsByGiveawayId(UUID giveawayId);
}
