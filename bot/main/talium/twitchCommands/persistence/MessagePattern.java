package talium.twitchCommands.persistence;

import jakarta.persistence.*;
import talium.twitchCommands.controller.MessagePatternDTO;

/**
 * A pattern that a message needs to match against in a {@link CommandEntity}
 */
@Entity
@Table(name = "sys-chatTrigger-patterns")
@IdClass(MessagePatternId.class)
public class MessagePattern {
    @Id @ManyToOne(fetch = FetchType.EAGER) @JoinColumn(name = "parent_trigger_id", nullable = false)
    @PrimaryKeyJoinColumn
    public CommandEntity parentTrigger;

    @Id
    public String pattern;
    public boolean isRegex;
    public boolean isVisible;
    public boolean isEnabled;

    public MessagePattern(String pattern, boolean isRegex, boolean isVisible, boolean isEnabled) {
        this.parentTrigger = null;
        this.pattern = pattern;
        this.isRegex = isRegex;
        this.isVisible = isVisible;
        this.isEnabled = isEnabled;
    }

    public MessagePattern(MessagePatternDTO dto) {
        this.parentTrigger = null;
        this.pattern = dto.pattern();
        this.isRegex = dto.isRegex();
        this.isVisible = dto.isVisible();
        this.isEnabled = dto.isEnabled();
    }

    protected MessagePattern() {

    }


    @Override
    public String toString() {
        return STR."MessagePattern[parentTrigger=\{parentTrigger != null ? parentTrigger.id : ""}, pattern=\{pattern}, isRegex=\{isRegex}, isVisible=\{isVisible}, isEnabled=\{isEnabled}\{']'}";
    }

    public MessagePatternDTO toMessagePatternDTO() {
        return new MessagePatternDTO(this);
    }
}
