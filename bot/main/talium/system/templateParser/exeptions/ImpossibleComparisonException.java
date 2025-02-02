package talium.system.templateParser.exeptions;

import talium.system.templateParser.statements.Equals;

public class ImpossibleComparisonException extends InterpretationException {
    public ImpossibleComparisonException(Object comparandA, Object comparandB, Equals operator) {
        super("Impossible comparing " + (comparandA == null ? "null" : comparandA.getClass().getCanonicalName()) + " and " + (comparandB == null ? "null" : comparandB.getClass().getCanonicalName()) + " via operator " + operator);
    }
}
