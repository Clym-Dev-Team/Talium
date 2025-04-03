package talium.stringTemplates;

import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.ListCrudRepository;

import java.util.List;

public interface TemplateRepo extends ListCrudRepository<Template, String> {

    @Query("SELECT t.id FROM Template t " +
    "WHERE t.id ilike %?1% " +
    "OR t.template ilike %?1% " +
    "OR t.messageColor ilike %?1%")
    List<String> searchAllBy(String search);


    Template getById(String id);
}
