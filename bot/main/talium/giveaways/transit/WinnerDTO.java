package talium.giveaways.transit;

import java.util.Optional;

/// Used for displaying Winners in giveaway Object and autosaving? Rejection Btn and comment for Winner. Unrelated to general giveaway save btn
public record WinnerDTO(
        String giveawayId,
        String userId,
        boolean rejected,
        Optional<String> comment
) {}
