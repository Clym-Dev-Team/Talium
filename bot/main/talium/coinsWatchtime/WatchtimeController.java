package talium.coinsWatchtime;

import com.github.twitch4j.helix.domain.User;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import talium.Out;
import talium.coinsWatchtime.chatter.Chatter;
import talium.coinsWatchtime.chatter.ChatterDTO;
import talium.coinsWatchtime.chatter.ChatterService;
import talium.coinsWatchtime.chatter.LeaderboardDTO;

import java.util.ArrayList;
import java.util.Map;
import java.util.TreeMap;

import static talium.TwitchBot.gson;

@RestController
@RequestMapping(value = "/watchtime", produces = "application/json")
public class WatchtimeController {
    private static final Logger logger = LoggerFactory.getLogger(WatchtimeController.class);
    private final ChatterService chatterService;

    @Autowired
    public WatchtimeController(ChatterService chatterService) {
        this.chatterService = chatterService;
    }

    @GetMapping("/top")
    String getTop() {
        var chatters = chatterService.getTopWatchtime();
        var dto = new ArrayList<LeaderboardDTO>();
        var userIdList = chatters.stream().map(chatter -> chatter.twitchUserId).toList();
        var twitchUserList = Out.Twitch.api.getUserById(userIdList);

        Map<String, String> usernameMap = new TreeMap<>();

        for (User user : twitchUserList) {
            usernameMap.put(user.getId(), user.getDisplayName());
        }
        for (Chatter chatter : chatters) {
            var username = usernameMap.get(chatter.twitchUserId);
            if (username == null) {
                logger.warn("UserID {} from chatterData could not be found on twitch", chatter.twitchUserId);
                continue;
            }
            dto.add(new LeaderboardDTO(username, chatter.watchtimeSeconds, chatter.coins));
        }
        return gson.toJson(dto);
    }

    @GetMapping("/username/{username}")
    ResponseEntity<String> getForUsername(@PathVariable String username) {
        var twitchUser = Out.Twitch.api.getUserByName(username);
        if (twitchUser.isEmpty()) {
            return ResponseEntity.notFound().build();
        }
        var chatter = chatterService.getChatterDataOrDefault(twitchUser.get().getId());
        return ResponseEntity.ok(gson.toJson(chatter.toChatterDto()));
    }

    @PostMapping("/update")
    HttpStatus post(@RequestBody String body) {
        var chatterDto = gson.fromJson(body, ChatterDTO.class);
        var dbChatter = chatterService.getChatterDataOrDefault(chatterDto.twitchUserId());
        int secondsSinceLastCoinsGain = 0;
        // use 0 as default if user is not yet in DB
        if (dbChatter != null) {
            secondsSinceLastCoinsGain = dbChatter.secondsSinceLastCoinsGain;
        }
        chatterService.save(chatterDto.toChatter(secondsSinceLastCoinsGain));
        return HttpStatus.OK;
    }
}