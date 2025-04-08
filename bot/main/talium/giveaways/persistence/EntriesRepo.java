package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;

public interface EntriesRepo extends ListCrudRepository<EntriesDAO, EntriesDAO.EntriesId> {
}
