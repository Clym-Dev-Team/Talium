package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;

interface EntriesRepo extends ListCrudRepository<EntriesDAO, EntriesDAO.EntriesId> {
}
