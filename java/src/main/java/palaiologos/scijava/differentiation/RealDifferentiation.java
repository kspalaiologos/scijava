package palaiologos.scijava.differentiation;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.integrator.RealFunction;

import java.util.Iterator;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class RealDifferentiation {
    public static SciFloat differentiate(MathContext mc, RealFunction f, SciFloat x, int n, DerivativeDirection dir, int additionalPrecision, boolean relativePrecision, boolean singular) {
        // TODO: Implement .scale() and .unscale() for SciFloat (divide/multiply by integer).
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
            vals = steps.mapToObj(k -> f.value(finalMc, SciFloat.add(finalMc, finalX, SciFloat.mul(finalMc, finalH, SciFloat.valueOf(finalMc, k)))));
        }
        // N-th forward difference operator.
        SciFloat d = SciFloat.ZERO;
        SciFloat b = n % 2 == 0 ? SciFloat.ONE : SciFloat.MINUS_ONE;
        int k = 0;
        for (Iterator<SciFloat> it = vals.iterator(); it.hasNext(); k++) {
            SciFloat kV = it.next();
            d = SciFloat.add(mc, d, SciFloat.mul(mc, kV, b));
            b = SciFloat.floor(mc, SciFloat.div(mc, SciFloat.mul(mc, b, SciFloat.valueOf(mc, k - n)), SciFloat.valueOf(mc, k + 1)));
        }
        return SciFloat.div(mc, d, SciFloat.pow(mc, SciFloat.valueOf(mc, n), norm));
    }
}
