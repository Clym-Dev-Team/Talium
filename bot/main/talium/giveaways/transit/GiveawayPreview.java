package talium.giveaways.transit;

import talium.giveaways.GiveawayStatus;
import talium.giveaways.persistence.GiveawayDAO;

import java.util.UUID;

public record GiveawayPreview(
        UUID id,
        GiveawayStatus status,
        String title,
        String notes
)
{
    public GiveawayPreview(GiveawayDAO gw) {
        this(gw.id(), gw.status(), gw.title(), gw.notes());
    }

    @Override
    public String toString() {
        return "GiveawayPreview{" +
                "id=" + id +
                ", status=" + status +
                ", title='" + title + '\'' +
                ", notes='" + notes + '\'' +
                '}';
    }
}
