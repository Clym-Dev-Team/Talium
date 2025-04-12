package talium.twitchCommands.persistence;

import java.io.Serializable;

public class MessagePatternId implements Serializable {
    String pattern;
    CommandEntity parentTrigger;

    public MessagePatternId(String pattern, CommandEntity parentTrigger) {
        this.pattern = pattern;
        this.parentTrigger = parentTrigger;
    }

    public MessagePatternId() {
    }

    @Override
    public final boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof MessagePatternId that)) return false;

        return pattern.equals(that.pattern) && parentTrigger.equals(that.parentTrigger);
    }

    @Override
    public int hashCode() {
        int result = pattern.hashCode();
        result = 31 * result + parentTrigger.hashCode();
        return result;
    }
}
