package talium.inputs.Twitch4J;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.twitch4j.helix.domain.Chatter;
import com.github.twitch4j.helix.domain.User;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

// maybe add no-op version of helix api and just switch them when they are assigned, that way we should be able to remove all these null checking methods
public class TwitchApi {

    /**
     * Sends a message in the chat specified by the twitchOutputToChannel Env with the bot account specified by the twitchBotAccountName env
     *
     * @param message the message text to send
     */
    public static void sendMessage(String message) {
        while (Twitch4JInput.chat == null) Thread.onSpinWait();
        Twitch4JInput.chat.sendMessage(Twitch4JInput.sendTo, message);
    }

    public static Optional<User> getUserById(String userId) {
        if (Twitch4JInput.helix == null) return Optional.empty();
        var user = Twitch4JInput.helix.getUsers(null, List.of(userId), null).execute().getUsers();
        if (user.isEmpty()) return Optional.empty();
        return Optional.ofNullable(user.getFirst());
    }

    public static List<User> getUserById(List<String> userId) {
        if (Twitch4JInput.helix == null) return new ArrayList<>();
        return Twitch4JInput.helix.getUsers(null, userId, null).execute().getUsers();
    }

    public static Optional<User> getUserByName(String username) {
        if (Twitch4JInput.helix == null) return Optional.empty();
        var user = Twitch4JInput.helix.getUsers(null, null, List.of(username)).execute().getUsers();
        if (user.isEmpty()) return Optional.empty();
        return Optional.ofNullable(user.getFirst());
    }

    public static List<Chatter> getUserList() {
        if (Twitch4JInput.helix == null) return new ArrayList<>();
        OAuth2Credential cred = Twitch4JInput.oAuth2Credential;
        return Twitch4JInput.helix
                .getChatters(cred.getAccessToken(), cred.getUserId(), cred.getUserId(), 1000, null)
                .execute()
                .getChatters();
    }

    public static boolean isOnline() {
        if (Twitch4JInput.helix == null) return false;
        OAuth2Credential cred = Twitch4JInput.oAuth2Credential;
        return Twitch4JInput.helix.getStreams(cred.getAccessToken(), null, null, null, null, null, null, null).execute().getStreams().size() == 1;
    }

}
