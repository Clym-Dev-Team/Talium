package talium.giveaways.transit;

import java.util.List;

public record GiveawayDrawDTO(
        List<WinnerDTO> winners
) {
}
