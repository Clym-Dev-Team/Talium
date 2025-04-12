package talium.coinsWatchtime.chatter;

import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;

@Repository
public interface ChatterRepo extends CrudRepository<Chatter, String> {

    List<Chatter> getAllByTwitchUserIdIn(List<String> twitchUserIds);

    List<Chatter> getAllByOrderByWatchtimeSecondsDesc();

    Chatter getByTwitchUserId(String twitchUserId);

    /**
     * This Method is specifically for entering giveaways, and it should not be used by anything else.
     * If in the future some other system needs to decrease the amount of coins a user has, the lock in the giveaway system needs to be exposed.
     * All decreases to a chatters coins need to get this ReadLock for the duration of the edit, because otherwise,
     * the coin & giveaway data may become corrupted/out of sync. This would lead to extra coins being given out, or to many
     * coins getting paid.
     * @apiNote THIS FUNCTION SHOULD ONLY BE USED FOR GIVEAWAY TICKETS. All calls to this function must abide by the Giveaway Lock.
     * @param twitchUserId
     * @param coins
     * @return
     */
    @Modifying
    @Transactional
    @Query("UPDATE Chatter SET coins = coins + ?2 WHERE coins >= ?2 AND twitchUserId = ?1")
    int addCoins(String twitchUserId, long coins);
}
