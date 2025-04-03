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

    public GiveawayTemplateDAO() {
        this.title = RandomStringUtils.randomAlphabetic(10);
        this.notes = RandomStringUtils.randomAlphabetic(10);
        this.commandPattern = "!" + RandomStringUtils.randomAlphabetic(10);
        this.ticketCost = RandomUtils.nextInt(50) * 10;
        this.maxTickets = RandomUtils.nextInt(50);
        this.allowRedrawOfUser = false;
        this.autoAnnounceWinner = false;
    }
}
