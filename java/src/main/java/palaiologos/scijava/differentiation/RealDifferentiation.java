package palaiologos.scijava.differentiation;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.integrator.RealFunction;

import java.util.Iterator;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RealDifferentiation {
    /**
     * Yield the value of the first derivative of a function at the given point.
     * Assumes that there is no singularity at thespecified point.
     * Additionally, assumes that the derivative exists and returns the central derivative.
     * The precision is slightly increased for the computations and not relative to the
     * magnitude of the argument.
     * @param mc The math context to use.
     * @param f The function to differentiate.
     * @param x The point at which to differentiate.
     * @return The value of the first derivative of the function at the given point.
     */
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x) {
        return differentiate(mc, f, x, 1);
    }

    /**
     * Differentiate a function at the specified point.
     * Assumes that there is no singularity at thespecified point.
     * Additionally, assumes that the derivative exists and returns the central derivative.
     * The precision is slightly increased for the computations and not relative to the
     * magnitude of the argument.
     * @param mc The math context.
     * @param f The function.
     * @param x The point.
     * @param n The order of the derivative.
     * @return The derivative.
     */
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x, int n) {
        return differentiate(mc, f, x, n, 10, false);
    }

    /**
     * Differentiate a given function at the specified point.
     * Assumes that there is no singularity at the specified point.
     * Additionally, assumes that the derivative exists (returns the central derivative)
     * @param mc The math context to use.
     * @param f The function to differentiate.
     * @param x The point at which to differentiate.
     * @param n The order of the derivative.
     * @param additionalPrecision The additional precision to use.
     * @param relativePrecision The relative precision to use.
     * @return The derivative.
     */
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x, int n, int additionalPrecision, boolean relativePrecision) {
        return differentiate(mc, f, x, n, DerivativeDirection.CENTRAL, additionalPrecision, relativePrecision);
    }

    /**
     * Differentiate a given function at the specified point.
     * Assumes that there is no singularity at the specified point.
     * @param mc The math context to use.
     * @param f The function to differentiate.
     * @param x The point at which to differentiate.
     * @param n The order of the derivative.
     * @param dir The direction of the finite difference.
     * @param additionalPrecision The additional precision to use.
     * @param relativePrecision The relative precision to use.
     * @return The derivative.
     */
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x, int n, DerivativeDirection dir, int additionalPrecision, boolean relativePrecision) {
        return differentiate(mc, f, x, n, dir, additionalPrecision, relativePrecision, false);
    }

    /**
     * Differentiate a given function at the specified point.
     * @param mc The math context to use.
     * @param f The function to differentiate.
     * @param x The point at which to differentiate.
     * @param n The order of the derivative.
     * @param dir The direction of the finite difference.
     * @param additionalPrecision The additional precision to use to account for perturbations.
     * @param relativePrecision Whether h should be chosen relative to x (magnitudal relationship).
     * @param singular Whether the function has a singularity at the given point, meaning that we should avoid
     *                 evaluating it precisely there.
     * @return The derivative at point.
     */
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x, int n, DerivativeDirection dir, int additionalPrecision, boolean relativePrecision, boolean singular) {
        if (n == 0 && !singular)
            return f.value(mc, x);
        int workprec = (mc.precision() + 2 * additionalPrecision) * (n + 1);
        MathContext orig = mc;
        mc = new MathContext(workprec, mc.roundingMode());
        int hextra = relativePrecision ? SciFloat.ceil(mc, SciFloat.log10(mc, x)).intValue(mc) : 0;
        SciFloat h = SciFloat.ldexp(mc, 1, -orig.precision() - additionalPrecision - hextra);
        SciFloat norm;
        IntStream steps;
        if (dir != DerivativeDirection.CENTRAL) {
            if (dir == DerivativeDirection.LEFT)
                h = SciFloat.neg(mc, h);
            norm = h;
            steps = IntStream.range(0, n + 1);
        } else {
            norm = SciFloat.mul(mc, h, SciFloat.TWO);
            steps = IntStream.iterate(-n, a -> a < n + 1, a -> a + 2);
        }
        if (singular)
            x = SciFloat.add(mc, x, SciFloat.mul(mc, h, SciFloat.HALF));
        Stream<SciFloat> vals;
        {
            // Dude, fuck Java!
            MathContext finalMc = mc;
            SciFloat finalX = x;
            SciFloat finalH = h;
            vals = steps.mapToObj(k -> f.value(finalMc, SciFloat.add(finalMc, finalX, SciFloat.scale(finalMc, finalH, k))));
        }
        // N-th forward difference operator.
        SciFloat d = SciFloat.ZERO;
        SciFloat b = n % 2 == 0 ? SciFloat.ONE : SciFloat.MINUS_ONE;
        int k = 0;
        for (Iterator<SciFloat> it = vals.iterator(); it.hasNext(); k++) {
            SciFloat kV = it.next();
            d = SciFloat.add(mc, d, SciFloat.mul(mc, kV, b));
            b = SciFloat.floor(mc, SciFloat.unscale(mc, SciFloat.scale(mc, b, k - n), k + 1));
        }
        return SciFloat.div(mc, d, SciFloat.pow(mc, norm, SciFloat.valueOf(mc, n)));
    }
}
