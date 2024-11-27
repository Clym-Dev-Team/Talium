package talium.system;

import java.time.Duration;

public class Timings {
    private final long startTime;
    private long endTime;

    public Timings() {
        startTime = System.nanoTime();
    }

    public void stop() {
        endTime = System.nanoTime();
    }

    public Duration eval() {
        if (endTime == 0) {
            stop();
        }
        return Duration.ofNanos(endTime - startTime);
    }
}
