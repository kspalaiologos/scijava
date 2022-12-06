package palaiologos.scijava.integrator;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;

/**
 * An univariate real function interface.
 *
 * @author Kamila Szewczyk
 */
public interface RealFunction {
    /**
     * Compute the value of the function.
     *
     * @param x the point for which the function value should be computed.
     * @return the value of the function.
     */
    SciFloat value(MathContext mc, SciFloat x);
}
