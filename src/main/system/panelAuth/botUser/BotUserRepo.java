package main.system.panelAuth.botUser;

import org.springframework.data.repository.CrudRepository;

public interface BotUserRepo extends CrudRepository<BotUser, Long> {
    BotUser findByUsername(String username);
}