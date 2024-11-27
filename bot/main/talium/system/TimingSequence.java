package talium.system;

import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class TimingSequence {
    record TimingStep(String name, long stamp) {
    }

    private boolean open = true;
    private final long startTime;
    private final List<TimingStep> steps;

    public TimingSequence() {
        startTime = System.nanoTime();
        steps = new ArrayList<>();
    }

    public void step(String name) {
        if (!open) {
            return;
        }
        steps.add(new TimingStep(name, System.nanoTime()));
    }

    public void print() {
        long endTime = System.nanoTime();
        open = false;
        var s = new StringJoiner(System.lineSeparator());
        s.add("----------------");
        s.add("Timings Report: ");
        long lastTimeStamp = startTime;
        for (TimingStep step : steps) {
            s.add(step.name + ": " + (step.stamp - lastTimeStamp) / 1000000f + "ms");
            lastTimeStamp = step.stamp;
        }
        s.add("Total: " + (endTime - startTime) / 1000000f + "ms");
        System.out.println(s);
    }
}
