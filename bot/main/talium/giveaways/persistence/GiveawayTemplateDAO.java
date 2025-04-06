package talium.giveaways.persistence;


import org.apache.commons.lang.RandomStringUtils;
import org.apache.commons.lang.math.RandomUtils;

public class GiveawayTemplateDAO {
    public String title;
    public String notes;
    public String commandPattern;
    public int ticketCost;
    public int maxTickets;
    public boolean allowRedrawOfUser;
    public boolean autoAnnounceWinner;

    public GiveawayTemplateDAO(String title, String notes, String commandPattern, int ticketCost, int maxTickets, boolean allowRedrawOfUser, boolean autoAnnounceWinner) {
        this.title = title;
        this.notes = notes;
        this.commandPattern = commandPattern;
        this.ticketCost = ticketCost;
        this.maxTickets = maxTickets;
        this.allowRedrawOfUser = allowRedrawOfUser;
        this.autoAnnounceWinner = autoAnnounceWinner;
    }

    public static GiveawayTemplateDAO Random() {
        return new GiveawayTemplateDAO(
                RandomStringUtils.randomAlphabetic(10),
                RandomStringUtils.randomAlphabetic(100),
                "!" + RandomStringUtils.randomAlphabetic(10),
                RandomUtils.nextInt(50) * 10,
                RandomUtils.nextInt(50),
                false,
                false
        );
    }

    public static GiveawayTemplateDAO Goldkette() {
        return new GiveawayTemplateDAO(
                "Goldkette",
                "Goldketten Gewinnspiel",
                "!goldkette",
                500,
                1,
                false,
                false
        );
    }
}
