package talium.giveaways.persistence;

import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;
import org.springframework.lang.Nullable;
import org.springframework.transaction.annotation.Transactional;
import talium.giveaways.GiveawayStatus;

import java.time.Instant;
import java.util.List;
import java.util.UUID;

public interface GiveawayRepo extends ListCrudRepository<GiveawayDAO, UUID> {

    List<GiveawayDAO> findAllByStatusIsNot(GiveawayStatus status);

    @Modifying
    @Transactional
    @Query("UPDATE GiveawayDAO SET status = ?2 WHERE id = ?1")
    void updateStatusById(UUID id, GiveawayStatus status);

    @Modifying
    @Transactional
    @Query("UPDATE GiveawayDAO SET title = ?2, notes = ?3, autoStart = ?4, autoEnd = ?5, ticketCost = ?6, maxTickets = ?7, allowRedrawOfUser = ?8, autoAnnounceWinner = ?9 WHERE id = ?1")
    void update(
            UUID id,
            String title,
            String notes,
            @Nullable Instant autoOpen,
            @Nullable Instant autoClose,
            int ticketCost,
            int maxTickets,
            boolean allowRedrawOfUser,
            boolean autoAnnounceWinner
    );
}
