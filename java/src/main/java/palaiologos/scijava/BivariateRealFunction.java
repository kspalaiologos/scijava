package palaiologos.scijava;

/**
 * A bivariate real function interface.
 *
 * @author Kamila Szewczyk
 */
public interface BivariateRealFunction {
    /**
     * Compute the value of the function.
     *
     * @param x the first point for which the function value should be computed.
     * @param y the second point for which the function value should be computed.
     * @return the value of the function.
     */
    SciFloat value(MathContext mc, SciFloat x, SciFloat y);
}
