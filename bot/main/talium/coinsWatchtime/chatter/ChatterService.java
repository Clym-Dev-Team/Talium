package talium.coinsWatchtime.chatter;

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

    /**
     * Subtract X amount of coins from user, if the user has at least X coins
     * @param twitchUserId user to subtract from
     * @param coins coins to substract
     * @return true if user had enough coins, and where subtracted, false if user had not enough coins
     */
    public boolean payCoinsFromChatter(String twitchUserId, int coins) {
        return chatterRepo.payCoins(twitchUserId, coins) == 1;
    }

}
