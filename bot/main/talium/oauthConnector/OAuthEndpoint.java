package talium.oauthConnector;

import com.github.philippheuer.credentialmanager.identityprovider.OAuth2IdentityProvider;
import org.apache.commons.lang.RandomStringUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.*;
import talium.TwitchBot;
import talium.UnexpectedShutdownException;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.*;

import static talium.TwitchBot.gson;

/**
 * Abstracts displaying the authorization url to the user, and receiving the redirect with the token.
 */
@Controller
public class OAuthEndpoint {

    private static String BOT_BASE_URL;
    private static String PANEL_BASE_URL;

    @Autowired
    public OAuthEndpoint(@Value("${serverBaseUrl}") String botBaseUrl, @Value("${panelBaseUrl}") String panelBaseUrl) {
        OAuthEndpoint.BOT_BASE_URL = botBaseUrl;
        OAuthEndpoint.PANEL_BASE_URL = panelBaseUrl;
    }

    private static class OauthRequest {
        public String state;
        public String url;
        public String accName;
        public String service;
        Optional<String> code;

        public OauthRequest(String state, String url, String accName, String service) {
            this.state = state;
            this.url = url;
            this.accName = accName;
            this.service = service;
            this.code = Optional.empty();
        }
    }

    private static final ArrayList<OauthRequest> requests = new ArrayList<>();

    /**
     * Get redirect url for a particular service. The url is fully formed and points to the real public host of the bot
     * @param service name of the service
     * @return the full url
     */
    public static String getRedirectUrl(String service) {
        return BOT_BASE_URL + "/auth/" + service;
    }

    /**
     * Get the public url to the panel oauth setup page.
     * @return the full url to the panel page
     */
    public static String getOauthSetupUrl() {
        return PANEL_BASE_URL + "/auth";
    }

    /**
     * Create a new Oauth Request by manually constructing the authorization url
     * @param accName name of the account to authorize
     * @param service name of the service the accounts lives on
     * @param authorizationUrl url to authorize the connection
     * @param state random character string that uniquely identifies this request
     * @apiNote <font color="red">This function blocks until request is completed or failed. This could take minutes to hours</font color="red">
     * @return if successfully, the oauth token
     */
    public static Optional<String> newAuthRequest(String accName, String service, String authorizationUrl, String state) {
        OauthRequest request = new OauthRequest(state, authorizationUrl, accName, service);
        requests.add(request);

        // wait for completion by user using web portal
        while (request.code.isEmpty() && !TwitchBot.requestedShutdown) {
            Thread.onSpinWait();
        }
        if (TwitchBot.requestedShutdown) {
            throw new UnexpectedShutdownException();
        }

        String code = request.code.get();
        requests.remove(request);
        return Optional.of(code);
    }

    /**
     * Create a new Oauth request via an OauthIdentityProvider.
     * Constructs a request state automatically.
     * @param accName name of the account to authorize
     * @param service name of the service the accounts lives on
     * @param iProvider identityProvider to use
     * @param scopes scopes to use with the identityProvider
     * @apiNote <font color="red">This function blocks until request is completed or failed. This could take minutes to hours</font color="red">
     * @return if successfully, the oauth token
     */
    public static Optional<String> newAuthRequest(String accName, String service, OAuth2IdentityProvider iProvider, ArrayList<String> scopes) {
        String state = RandomStringUtils.randomAlphanumeric(30);
        String auth_url = iProvider.getAuthenticationUrl(Collections.singletonList(scopes), state);
        return newAuthRequest(accName, service, auth_url, state);
    }

    /**
     * Create a new Oauth request via an OauthIdentityProvider.
     * Constructs a request state automatically.
     * @param accName name of the account to authorize
     * @param service name of the service the accounts lives on
     * @param iProvider identityProvider to use
     * @param scopes scopes to use with the identityProvider
     * @apiNote <font color="red">This function blocks until request is completed or failed. This could take minutes to hours</font color="red">
     * @return if successfully, the oauth token
     */
    public static Optional<String> newAuthRequest(String accName, String service, OAuth2IdentityProvider iProvider, String... scopes) {
        String state = RandomStringUtils.randomAlphanumeric(30);
        List<Object> objectList = Collections.singletonList(Arrays.stream(scopes).toList());
        String auth_url = iProvider.getAuthenticationUrl(objectList, state);
        return newAuthRequest(accName, service, auth_url, state);
    }


    @RequestMapping("/auth/{service}")
    public String oAuthEndpoint(@PathVariable String service, @RequestParam(required = false) String code, @RequestParam(required = false) String scope, @RequestParam String state, @RequestParam(required = false) String error, @RequestParam(required = false) String error_description, Model model) {
        //If the state is not equal to the state of our request, the response is not ours
        var s = requests.stream().filter(r -> r.state.equals(state)).findFirst();
        if (s.isEmpty()) {
            return STR."redirect:\{PANEL_BASE_URL}?success=false&error=" + URLEncoder.encode("This oauth was never requested from the app", StandardCharsets.UTF_8);
        } else if (error != null) {
            return STR."redirect:\{PANEL_BASE_URL}?success=false&error=" + URLEncoder.encode(error_description, StandardCharsets.UTF_8);
        }

        s.get().code = Optional.of(code);
        return STR."redirect:\{PANEL_BASE_URL}?success=true";
    }

    @GetMapping("/setup/auth/list")
    @ResponseBody
    public String oauthList() {
        record OauthDTO(String url, String accName, String service) {}
        var r = requests.stream().map(o -> new OauthDTO(o.url, o.accName, o.service)).toList();
        return gson.toJson(r);
    }
}
