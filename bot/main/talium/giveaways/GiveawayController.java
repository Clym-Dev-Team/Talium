package talium.giveaways;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.*;
import talium.giveaways.persistence.GiveawayService;
import talium.giveaways.transit.GiveawayDTO;
import talium.giveaways.transit.GiveawaySaveDTO;

import java.time.Instant;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

@RestController
@RequestMapping("/giveaway")
public class GiveawayController {
    private final Gson gson = new GsonBuilder().serializeNulls().create();

    GiveawayService giveawayService;

    @Autowired
    public GiveawayController(GiveawayService giveawayService) {
        this.giveawayService = giveawayService;
    }

    @PostMapping("/save/{gwId}")
    public HttpStatus create(@RequestBody String body, @PathVariable UUID gwId) {
        var giveaway = gson.fromJson(body, GiveawaySaveDTO.class);
        System.out.println("Giveaway");
        System.out.println(giveaway);
        //TODO save all to DB
        //TODO update command for GW
        //TODO trigger possible correction of amount and ticket amounts
        //TODO trigger update of autostart/close timers
        return HttpStatus.CREATED;
    }

    //TODO make create into into save
    // make new, create from template endpoint

    //TODO open close GW
    //TODO archive endpoint
    //TODO draw endpoint
    //TODO winner save endpoint
    //TODO refund all tickets

    @GetMapping
    public String getActive() {
        var g = new GiveawayDTO(UUID.randomUUID(), "test Title", "", Instant.MIN, Instant.now(), GiveawayStatus.CREATED, "giveaways.test1", "!testGW", "you failed", Optional.empty(), Optional.empty(), 3, 10, false, false, List.of(), List.of());
        return gson.toJson(g);
    }

    //TODO needed: Object to Manage the relationship with Commands. Update them, create and Set Command when needed,
    // delete Commands, and register and handle Callbacks from Commands
    //TODO needed: A Object to handle Giveaway AutostartTimers, schedule them, have interrupt to check if something changed on gw save,
    // trigger action when timer up
    //TODO concept: Do we still want to do the automatic action (autostart/close) when the status of the giveaway changed in the meantime?
    // like, if you manually opened and closed it in between, maybe that should cancel any automatic actions?
    // Said differently: It is probably appropriate that, if a change in status is done manually, autostart and autoclose times should be removed (null)
    //TODO archive probably needs to be its on btn in the panel and actions, (that is just greyed out and disabled when GW is running)


    /* GW actions that do not originate from the panel:
        - automatic actions like: autostart/close
        - entering a giveaway

    */
}
