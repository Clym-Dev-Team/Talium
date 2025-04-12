package talium.giveaways;

import com.github.twitch4j.helix.domain.User;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import talium.Out;
import talium.giveaways.persistence.GiveawayRepo;
import talium.giveaways.transit.*;

import java.net.URI;
import java.util.List;
import java.util.UUID;

import static talium.TwitchBot.gson;

@RestController
@RequestMapping("/giveaway")
public class GiveawayController {
    private final GiveawayRepo giveawayRepo;

    GiveawayService giveawayService;

    @Autowired
    public GiveawayController(GiveawayService giveawayService, GiveawayRepo giveawayRepo) {
        this.giveawayService = giveawayService;
        this.giveawayRepo = giveawayRepo;
    }

    @GetMapping("/templates")
    public String getGiveawayTemplates() {
        return gson.toJson(List.of(
                new GiveawayTemplateDTO("BLANK", "blank"),
                new GiveawayTemplateDTO("Goldkette", "goldkette"),
                 new GiveawayTemplateDTO("Random", "random"))
        );
    }

    @PostMapping("/fromTemplate/{templateId}")
    public ResponseEntity<String> getGiveawayTemplates(@PathVariable String templateId) {
        var body = gson.toJson(giveawayService.createFromTemplate(templateId));
        return ResponseEntity.status(HttpStatus.CREATED).body(body);
    }

    @PostMapping("/create")
    public ResponseEntity<String> save(@RequestBody String body) {
        var toSave = gson.fromJson(body, GiveawaySaveDTO.class);
        var dao = giveawayService.saveNewGiveaway(toSave);
        return ResponseEntity.created(URI.create("/giveaway/get/" + dao.id())).build();
    }

    @PostMapping("/save/{gwId}")
    public ResponseEntity<String> save(@RequestBody String body, @PathVariable UUID gwId) {
        var toSave = gson.fromJson(body, GiveawaySaveDTO.class);
        var old = giveawayRepo.findById(gwId);
        if (old.isEmpty()) {
            return ResponseEntity.status(HttpStatus.NOT_FOUND).body("Giveaway with that id not found, cannot update");
        }
        try {
            giveawayService.updateGiveaway(toSave, old.get());
        } catch (Exception e) {
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Could not update giveaway");
        }
        return ResponseEntity.ok("");
    }

    @GetMapping("/get/{gwId}")
    public ResponseEntity<String> getGiveaway(@PathVariable UUID gwId) {
        var entity = giveawayRepo.findById(gwId);
        if (entity.isEmpty()) {
            return new ResponseEntity<>("No Giveaway with Id: " + gwId + " could be found", HttpStatus.NOT_FOUND);
        }
        var gw = entity.get();
        var dto = new GiveawayDTO(gw.id(), gw.title(), gw.notes(), gw.createdAt().toString(), gw.lastUpdatedAt().toString(), gw.status(), gw.commandPattern(), gw.autoStart() != null ? gw.autoStart().toString() : null, gw.autoEnd() != null ? gw.autoEnd().toString() : null, gw.ticketCost(), gw.maxTickets(), gw.allowRedrawOfUser(), gw.autoAnnounceWinner(), gw.ticketList().stream().map(ent -> {
            var name = Out.Twitch.api.getUserById(ent.userId()).map(User::getId).orElse(ent.userId());
            return new EntryDTO(ent.userId(), name, ent.tickets());
        }).toList(), gw.winners().stream().map(win -> {
            var name = Out.Twitch.api.getUserById(win.userId()).map(User::getId).orElse(win.userId());
            return new WinnerDTO(name, win.userId(), win.rejected(), win.comment());
        }).toList());
        return ResponseEntity.ok(gson.toJson(dto));
    }

    @GetMapping("/listActive")
    public String getGiveaways() {
        return gson.toJson(giveawayRepo.findAllByStatusIsNot(GiveawayStatus.ARCHIVED).stream().map(GiveawayPreview::new).toList());
    }

    @PostMapping("/action/{gwId}/{action}")
    public ResponseEntity<String> action(@PathVariable UUID gwId, @PathVariable String action) {
        var gwOption = giveawayRepo.findById(gwId);
        if (gwOption.isEmpty()) {
            return ResponseEntity.status(HttpStatus.NOT_FOUND).body("Giveaway with that id not found, cannot action");
        }
        var gw = gwOption.get();
        switch (action.toLowerCase()) {
            case "open" -> {
                if (gw.status() != GiveawayStatus.RUNNING) {
                    giveawayRepo.updateStatusById(gw.id(), GiveawayStatus.RUNNING);
                }
                return ResponseEntity.ok("");
            }
            case "close" -> {
                if (gw.status() == GiveawayStatus.ARCHIVED) {
                    return ResponseEntity.badRequest().body("Giveaway cannot be set to Closed, unarchive first");
                }
                if (gw.status() == GiveawayStatus.RUNNING) {
                    giveawayRepo.updateStatusById(gw.id(), GiveawayStatus.PAUSED);
                }
                return ResponseEntity.ok("");
            }
            case "draw" -> {
                if (gw.status() == GiveawayStatus.ARCHIVED) {
                    return ResponseEntity.badRequest().body("Unable to draw winner, unarchive first");
                }
                if (gw.status() == GiveawayStatus.RUNNING) {
                    giveawayRepo.updateStatusById(gw.id(), GiveawayStatus.PAUSED);
                }
                giveawayService.draw(gw);
                return ResponseEntity.ok("");
            }
            case "refundall" -> {
                if (gw.status() != GiveawayStatus.PAUSED) {
                    return ResponseEntity.badRequest().body("Unable to refund tickets, pause giveaway first");
                }
                try {
                    giveawayService.refundAllTickets(gw);
                } catch (Exception e) {
                    return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("Unable to refund tickets");
                }
                return ResponseEntity.ok("");
            }
            case "archive" -> {
                if (gw.status() != GiveawayStatus.PAUSED) {
                    return ResponseEntity.badRequest().body("Unable to unarchive giveaway, pause giveaway first");
                }
                giveawayService.archive(gw);
            }
            case "unarchive" -> {
                if (gw.status() != GiveawayStatus.ARCHIVED) {
                    return ResponseEntity.badRequest().body("Unable to unarchive giveaway. Giveaway is not archived");
                }
                giveawayService.unarchive(gw);
            }
            case "delete" -> {
                if (gw.status() != GiveawayStatus.ARCHIVED && gw.ticketList().isEmpty()) {
                    return ResponseEntity.badRequest().body("Can only delete archived giveaways without tickets");
                }
                giveawayService.deleteArchived(gw);
            }
            default -> {
                return ResponseEntity.status(HttpStatus.BAD_REQUEST).body("Invalid action for this endpoint: " + action);
            }
        }
        return ResponseEntity.ok("");
    }

    //TODO winner save endpoint
}
