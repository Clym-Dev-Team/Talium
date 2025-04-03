package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;
import talium.giveaways.GiveawayStatus;

import java.util.List;
import java.util.UUID;

public interface GiveawayRepo extends ListCrudRepository<GiveawayDAO, UUID> {

    List<GiveawayDAO> findAllByStatusIsNot(GiveawayStatus status);
}
