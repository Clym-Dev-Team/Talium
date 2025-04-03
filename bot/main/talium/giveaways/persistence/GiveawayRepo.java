package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;

public interface GiveawayRepo extends ListCrudRepository<GiveawayDAO, Integer> {
}
