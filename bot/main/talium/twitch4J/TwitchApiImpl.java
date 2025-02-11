package talium.twitch4J;

import com.github.philippheuer.credentialmanager.domain.OAuth2Credential;
import com.github.twitch4j.chat.TwitchChat;
import com.github.twitch4j.common.exception.UnauthorizedException;
import com.github.twitch4j.helix.TwitchHelix;
import com.github.twitch4j.helix.domain.Chatter;
import com.github.twitch4j.helix.domain.User;
import com.netflix.hystrix.exception.HystrixRuntimeException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import talium.TwitchBot;

import java.util.List;
import java.util.Optional;

// maybe add no-op version of helix api and just switch them when they are assigned, that way we should be able to remove all these null checking methods
public class TwitchApiImpl implements TwitchApi {

    private static final Logger logger = LoggerFactory.getLogger(TwitchApiImpl.class);

    TwitchHelix helix;
    TwitchChat chat;
    String sendToChannel;

    public TwitchApiImpl(TwitchHelix helix, TwitchChat chat, String sendToChannel) {
        this.helix = helix;
        this.chat = chat;
        this.sendToChannel = sendToChannel;
    }

    // wip: use higher-order-function to reduce boilerplate
    private interface Call<T> {
        T call(TwitchHelix helix);
    }

    private static<T> T doHelixCall(Call<T> call) {
        try {
            return call.call(Twitch4JInput.helix);
        }  catch (HystrixRuntimeException e) {
            if (!(e.getCause() instanceof UnauthorizedException)) {
                // ehh, log and throw i guess
                throw new RuntimeException(e);
            }
            logger.warn("Twitch Credentials invalid, trying to reconnect to twitch!");
            var success = TwitchBot.reconnectTwitch();
            if (!success) {
                logger.error("Failed to reconnect twitch!");
                //TODO make this checked exception, with cause from reconnectTwitch() from TwitchInput.startup()
                throw new RuntimeException("Failed to reconnect to twitch!");
            }
            return call.call(Twitch4JInput.helix);
        } catch (Exception e) {
            // ehh, log and throw i guess
            throw new RuntimeException(e);
        }
    }

    @Override
    public boolean isPlaceHolder() {
        return false;
    }

    /**
     * Sends a message in the chat specified by the twitchOutputToChannel Env with the bot account specified by the twitchBotAccountName env
     *
     * @param message the message text to send
     */
    @Override
    public void sendMessage(String message) {
        chat.sendMessage(sendToChannel, message);
    }

    @Override
    public Optional<User> getUserById(String userId) {
        var user = doHelixCall(h -> h.getUsers(null, List.of(userId), null).execute().getUsers());
        if (user.isEmpty()) return Optional.empty();
        return Optional.ofNullable(user.getFirst());
    }

    @Override
    public List<User> getUserById(List<String> userId) {
        return doHelixCall(h -> h.getUsers(null, userId, null).execute().getUsers());
    }

    @Override
    public Optional<User> getUserByName(String username) {
        var user = doHelixCall(h -> h.getUsers(null, null, List.of(username)).execute().getUsers());
        if (user.isEmpty()) return Optional.empty();
        return Optional.ofNullable(user.getFirst());
    }

    @Override
    public List<Chatter> getUserList() {
        OAuth2Credential cred = Twitch4JInput.oAuth2Credential;
        return doHelixCall(h -> h
                .getChatters(cred.getAccessToken(), cred.getUserId(), cred.getUserId(), 1000, null)
                .execute()
                .getChatters());
    }

    @Override
    public boolean isOnline() {
        OAuth2Credential cred = Twitch4JInput.oAuth2Credential;
        return doHelixCall(h -> h.getStreams(cred.getAccessToken(), null, null, null, null, null, null, null).execute().getStreams()).size() == 1;
    }

}
