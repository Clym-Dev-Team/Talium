package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;

interface GiveawayRepo extends ListCrudRepository<GiveawayDAO, Integer> {
}
