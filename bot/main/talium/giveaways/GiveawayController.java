package talium.giveaways;

import com.github.twitch4j.helix.domain.User;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import talium.Out;
import talium.giveaways.persistence.GiveawayRepo;
import talium.giveaways.transit.*;

import java.util.*;

@RestController
@RequestMapping("/giveaway")
public class GiveawayController {
    private final Gson gson = new GsonBuilder().serializeNulls().create();
    private final GiveawayRepo giveawayRepo;

    GiveawayService giveawayService;

    @Autowired
    public GiveawayController(GiveawayService giveawayService, GiveawayRepo giveawayRepo) {
        this.giveawayService = giveawayService;
        this.giveawayRepo = giveawayRepo;
    }

    @GetMapping("/templates")
    public String getGiveawayTemplates() {
        return gson.toJson(List.of(new GiveawayTemplateDTO("BLANK", "blank")));
    }

    @PostMapping("/fromTemplate/{templateId}")
    public ResponseEntity<UUID> getGiveawayTemplates(@PathVariable String templateId) {
        var id = giveawayService.createFromTemplate(templateId);
        return ResponseEntity.status(HttpStatus.CREATED).body(id);
    }

    @PostMapping("/save/{gwId}")
    public HttpStatus save(@RequestBody String body, @PathVariable UUID gwId) {
        var giveaway = gson.fromJson(body, GiveawaySaveDTO.class);
        System.out.println("Giveaway");
        System.out.println(giveaway);
        //TODO save all to DB
        //TODO update command for GW
        //TODO trigger possible correction of amount and ticket amounts
        //TODO trigger update of autostart/close timers
        return HttpStatus.CREATED;
    }

    //TODO open close GW
    //TODO archive endpoint
    //TODO draw endpoint
    //TODO winner save endpoint
    //TODO refund all tickets


    @GetMapping("/get/{gwId}")
    public ResponseEntity<String> getGiveaway(@PathVariable UUID gwId) {
        var entity = giveawayRepo.findById(gwId);
        if (entity.isEmpty()) {
            return new ResponseEntity<>("No Giveaway with Id: " + gwId + " could be found", HttpStatus.NOT_FOUND);
        }
        var gw = entity.get();
        var dto = new GiveawayDTO(
                gw.id(),
                gw.title(),
                gw.notes(),
                gw.createdAt().toString(),
                gw.lastUpdatedAt().toString(),
                gw.status(),
                gw.command().patterns.getFirst().pattern,
                gw.autoStart() != null ? gw.autoStart().toString() : null,
                gw.autoEnd() != null ? gw.autoEnd().toString() : null,
                gw.ticketCost(),
                gw.maxTickets(),
                gw.allowRedrawOfUser(),
                gw.autoAnnounceWinner(),
                gw.ticketList().stream().map(ent -> {
                    var name = Out.Twitch.api.getUserById(ent.userId()).map(User::getId).orElse(ent.userId());
                    return new EntryDTO(ent.userId(), name, ent.tickets());
                }).toList(),
                gw.winners().stream().map(win -> {
                    var name = Out.Twitch.api.getUserById(win.userId()).map(User::getId).orElse(win.userId());
                    return new WinnerDTO(name, win.userId(), win.rejected(), win.comment());
                }).toList()
        );
        return ResponseEntity.ok(gson.toJson(dto));
    }

    @GetMapping("/listActive")
    public String getGiveaways() {
        return gson.toJson(giveawayRepo
                .findAllByStatusIsNot(GiveawayStatus.ARCHIVED)
                .stream().map(GiveawayPreview::new).toList());
    }

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
