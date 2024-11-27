package talium.system.coinsWatchtime.chatter;

import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface ChatterRepo extends CrudRepository<Chatter, String> {

    List<Chatter> getAllByTwitchUserIdIn(List<String> twitchUserIds);

    List<Chatter> getAllByOrderByWatchtimeSecondsDesc();

    Chatter getByTwitchUserId(String twitchUserId);
}
