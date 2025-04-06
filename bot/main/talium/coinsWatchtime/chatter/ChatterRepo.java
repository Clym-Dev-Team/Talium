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

    @Modifying
    @Transactional
    @Query("UPDATE Chatter SET coins = coins - ?1 WHERE coins >= ?1 AND twitchUserId = ?2")
    int payCoins(String twitchUserId, int coins);
}
