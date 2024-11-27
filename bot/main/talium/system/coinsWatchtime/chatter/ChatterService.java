package talium.system.coinsWatchtime.chatter;

import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ChatterService {
    ChatterRepo chatterRepo;

    public ChatterService(ChatterRepo chatterRepo) {
        this.chatterRepo = chatterRepo;
    }

    public List<Chatter> getChattersOrDefault(List<String> chatterIds) {
        var chatters = chatterRepo.getAllByTwitchUserIdIn(chatterIds);
        var dbChatterIds = chatters.stream().map(chatter -> chatter.twitchUserId).toList();

        // get all chatter Ids that were not found in the DB and insert default object for them, so that the resulting is ist complete
        var chatterIdsCopy = new java.util.ArrayList<>(chatterIds);
        chatterIdsCopy.removeAll(dbChatterIds);
        for (var chatterId : chatterIdsCopy) {
            chatters.add(new Chatter(chatterId));
        }
        return chatters;
    }

    public void saveAll(List<Chatter> chatters) {
        chatterRepo.saveAll(chatters);
    }

    public void save(Chatter chatters) {
        chatterRepo.save(chatters);
    }

    public List<Chatter> getTopWatchtime() {
        return chatterRepo.getAllByOrderByWatchtimeSecondsDesc();
    }

    public Chatter getDataForChatter(String userId) {
        var dbResult = chatterRepo.getByTwitchUserId(userId);
        if (dbResult == null) {
            return new Chatter(userId);
        }
        return dbResult;
    }

}
