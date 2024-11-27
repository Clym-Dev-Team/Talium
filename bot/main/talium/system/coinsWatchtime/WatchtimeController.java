package talium.system.coinsWatchtime;

import com.github.twitch4j.helix.domain.User;
import com.google.gson.Gson;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import talium.inputs.Twitch4J.TwitchApi;
import talium.system.coinsWatchtime.chatter.Chatter;
import talium.system.coinsWatchtime.chatter.ChatterDTO;
import talium.system.coinsWatchtime.chatter.ChatterService;
import talium.system.coinsWatchtime.chatter.LeaderboardDTO;

import java.util.ArrayList;
import java.util.Map;
import java.util.TreeMap;

@RestController
@RequestMapping(value = "/watchtime", produces = "application/json")
public class WatchtimeController {
    private static final Logger logger = LoggerFactory.getLogger(WatchtimeController.class);
    private final ChatterService chatterService;
    private final Gson gson = new Gson();

    @Autowired
    public WatchtimeController(ChatterService chatterService) {
        this.chatterService = chatterService;
    }

    @GetMapping("/top")
    String getTop() {
//        var t = new TimingSequence();
        var chatters = chatterService.getTopWatchtime();
//        t.step("DB");
        var dto = new ArrayList<LeaderboardDTO>();
        var userIdList = chatters.stream().map(chatter -> chatter.twitchUserId).toList();
//        t.step("TRANSFORM LIST");
        var twitchUserList = TwitchApi.getUserById(userIdList);
//        t.step("TWITCH API");

        Map<String, String> usernameMap = new TreeMap<>();

        for (User user : twitchUserList) {
            usernameMap.put(user.getId(), user.getId());
        }
//        t.step("TRANSFORM SET");
        for (Chatter chatter : chatters) {
            var username = usernameMap.get(chatter.twitchUserId);
            if (username == null) {
                logger.warn("UserID {} from chatterData could not be found on twitch", chatter.twitchUserId);
                continue;
            }
            dto.add(new LeaderboardDTO(username, chatter.watchtimeSeconds, chatter.coins));
        }
//        t.step("MAP");
        String json = gson.toJson(dto);
//        t.step("json");
//        t.print();
        return json;
    }

    @GetMapping("/username/{username}")
    ResponseEntity<String> getForUsername(@PathVariable String username) {
        var twitchUser = TwitchApi.getUserByName(username);
        if (twitchUser.isEmpty()) {
            return ResponseEntity.notFound().build();
        }
        var chatter = chatterService.getDataForChatter(twitchUser.get().getId());
        return ResponseEntity.ok(gson.toJson(chatter.toChatterDto()));
    }

    @PostMapping("/update")
    HttpStatus post(@RequestBody String body) {
        var chatterDto = gson.fromJson(body, ChatterDTO.class);
        var dbChatter = chatterService.getDataForChatter(chatterDto.twitchUserId());
        int secondsSinceLastCoinsGain = 0;
        // use 0 as default if user is not yet in DB
        if (dbChatter != null) {
            secondsSinceLastCoinsGain = dbChatter.secondsSinceLastCoinsGain;
        }
        chatterService.save(chatterDto.toChatter(secondsSinceLastCoinsGain));
        return HttpStatus.OK;
    }
}