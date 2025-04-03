package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;
import talium.giveaways.GiveawayStatus;

import java.util.List;

public interface GiveawayRepo extends ListCrudRepository<GiveawayDAO, Integer> {

    List<GiveawayDAO> findAllByStatusIsNot(GiveawayStatus status);
}
