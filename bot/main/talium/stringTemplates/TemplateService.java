package talium.stringTemplates;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;
import talium.twitchCommands.persistence.TriggerRepo;

import java.util.List;
import java.util.Optional;

@Service
public class TemplateService {
    private final TriggerRepo triggerRepo;
    TemplateRepo repo;

    @Autowired
    public TemplateService(TemplateRepo repo, TriggerRepo triggerRepo) {
        this.repo = repo;
        this.triggerRepo = triggerRepo;
    }

    public List<Template> searchBy(String search) {
        var ids = repo.searchAllBy(search);
        return repo.findAllById(ids);
    }

    public void delete(String id) {
        repo.deleteById(id);
    }

    public Optional<Template> getTemplateById(String id) {
        return repo.findById(id);
    }

    public Optional<Template> getTemplateByCommandId(String commandId) {
        var command = triggerRepo.findById(commandId);
        if (command.isEmpty()) {
            return Optional.empty();
        }
        return Optional.ofNullable(command.get().template);
    }

    public List<Template> getAllTemplates() {
        return repo.findAll();
    }

    public Template save(Template template) {
        return repo.save(template);
    }

    public Template saveIfAbsent(Template template) {
        if (repo.existsById(template.id)) {
            return repo.getById(template.id);
        }
        return save(template);
    }
}
