package talium.twitchCommands.persistence;

import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;
import org.springframework.transaction.annotation.Transactional;

@Repository
public interface MessagePatternRepo extends CrudRepository<MessagePattern, MessagePatternId> {

    @Transactional
    void deleteAllByParentTrigger(CommandEntity parentTrigger);

}
