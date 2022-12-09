package palaiologos.scijava.sum;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.differentiation.RealDifferentiation;
import palaiologos.scijava.integrator.RealFunction;
import palaiologos.scijava.integrator.RealGaussLegendreIntegrator;
import palaiologos.scijava.integrator.RealTanhSinhIntegrator;
import palaiologos.scijava.util.Pair;

import java.util.Iterator;
import java.util.Objects;
import java.util.function.Supplier;
import java.util.stream.Stream;

public class EulerMaclaurinSum {
    public static Pair<SciFloat, SciFloat> sum(MathContext mc, RealFunction f, SciFloat a, SciFloat b, Stream<SciFloat> aDiffs, Stream<SciFloat> bDiffs, Pair<SciFloat, SciFloat> integral) {
        final int M = 10000;
        SciFloat err = SciFloat.ZERO;
        SciFloat prev = SciFloat.ZERO;
        SciFloat s = SciFloat.ZERO;
        SciFloat eps = SciFloat.ldexp(mc, 1, -mc.precision() - 4);
        Stream<SciFloat> ad, bd;

        if(a.isInf() && a.lt(SciFloat.ZERO)) {
            ad = Stream.generate(() -> SciFloat.ZERO);
        } else ad = Objects.requireNonNullElseGet(aDiffs, () -> Stream.generate(new Supplier<>() {
            private int d = 0;

            @Override
            public SciFloat get() {
                return RealDifferentiation.differentiate(mc, f, a, d++);
            }
        }));

        if(b.isInf() && b.gt(SciFloat.ZERO)) {
            bd = Stream.generate(() -> SciFloat.ZERO);
        } else bd = Objects.requireNonNullElseGet(bDiffs, () -> Stream.generate(new Supplier<>() {
            private int d = 0;

            @Override
            public SciFloat get() {
                return RealDifferentiation.differentiate(mc, f, b, d++);
            }
        }));

        MathContext wmc = new MathContext(mc.precision() + 10, mc.roundingMode());

        Iterator<SciFloat> aIter = ad.iterator(), bIter = bd.iterator();
        for(int k = 0; ; k++) {
            SciFloat da = aIter.next(), db = bIter.next();
            if(k % 2 == 1) {
                SciFloat term = SciFloat.sub(wmc, db, da);
                term = SciFloat.mul(wmc, term, SciFloat.bernoulli(wmc, k + 1));
                term = SciFloat.div(wmc, term, SciFloat.factorial(wmc, k + 1));
                SciFloat mag = SciFloat.abs(wmc, term);
                if(k > 4 && mag.lt(eps)) {
                    s = SciFloat.add(wmc, s, term);
                    break;
                } else if(k > 4 && SciFloat.div(wmc, SciFloat.abs(wmc, prev), mag).lt(SciFloat.TEN)) {
                    err = SciFloat.add(wmc, err, mag);
                    break;
                } else {
                    s = SciFloat.add(wmc, s, term);
                    prev = term;
                }
            }
        }

        if(a.neq(SciFloat.NINF)) {
            s = SciFloat.add(wmc, s, SciFloat.div(mc, f.value(mc, a), SciFloat.TWO));
        }

        if(b.neq(SciFloat.INF)) {
            s = SciFloat.add(wmc, s, SciFloat.div(mc, f.value(mc, b), SciFloat.TWO));
        }

        if(integral == null) {
            Pair<SciFloat, SciFloat> I = RealGaussLegendreIntegrator.quad(wmc, f, new SciFloat[]{a, b});
            s = SciFloat.add(wmc, s, I.left);
            err = SciFloat.add(wmc, err, I.right);
        } else {
            s = SciFloat.add(wmc, s, integral.left);
            err = SciFloat.add(wmc, err, integral.right);
        }

        return new Pair<>(SciFloat.add(mc, s, SciFloat.ZERO), SciFloat.add(mc, err, SciFloat.ZERO));
    }
}
