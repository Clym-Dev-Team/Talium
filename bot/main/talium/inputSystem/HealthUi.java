package talium.inputSystem;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ResponseBody;

import static talium.TwitchBot.gson;

@Controller
public class HealthUi {

    @GetMapping("/health/json")
    @ResponseBody
    public String healthJson() {
        return gson.toJson(HealthManager.allStatuses());
    }
}
