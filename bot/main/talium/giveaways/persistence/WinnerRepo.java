package talium.giveaways.persistence;

import org.springframework.data.repository.ListCrudRepository;

interface WinnerRepo extends ListCrudRepository<WinnersDAO, EntriesDAO.EntriesId> {

}
