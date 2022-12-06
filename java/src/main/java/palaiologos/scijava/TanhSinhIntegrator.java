package palaiologos.scijava;

import palaiologos.scijava.util.ConcurrentLRUCache;
import palaiologos.scijava.util.Pair;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

public final class TanhSinhIntegrator {
    private static final int LRU_SIZE = 256;

    private int guessDegree(MathContext mc) {
        return 6 + Math.max(0, (int) Math.ceil(Math.log(mc.precision() / 30.0) / Math.log(2)));
    }

    private static native SciFloat[][] transformNodes(int precision, SciFloat[][] orig, SciFloat a, SciFloat b);
    private static native SciFloat[][] getNodes(int precision, int degree);

    private static ConcurrentLRUCache<IntegratorProperties, SciFloat[][]> nodeCache = new ConcurrentLRUCache<>(LRU_SIZE);

    private static SciFloat[][] getNodes(MathContext mc, IntegratorProperties properties) {
        SciFloat[][] nodes = nodeCache.get(properties);
        if (nodes == null) {
            nodes = getNodes(mc.precision() + 20, properties.degree);
            nodes = transformNodes(mc.precision() + 20, nodes, properties.a, properties.b);
            nodeCache.put(properties, nodes);
        }
        return nodes;
    }

    public static void dropCaches() {
        nodeCache = new ConcurrentLRUCache<>(LRU_SIZE);
    }

    private static SciFloat estimateError(MathContext mc, SciFloat epsilon, List<SciFloat> results) {
        if (results.size() == 2) {
            return SciFloat.abs(mc, SciFloat.sub(mc, results.get(0), results.get(1)));
        }
        if(results.get(results.size() - 1).equals(results.get(results.size() - 2)) && results.get(results.size() - 1).equals(results.get(results.size() - 3))) {
            return SciFloat.ZERO;
        }
        SciFloat D1 = SciFloat.log10(mc, SciFloat.abs(mc, SciFloat.sub(mc, results.get(results.size() - 1), results.get(results.size() - 2))));
        SciFloat D2 = SciFloat.log10(mc, SciFloat.abs(mc, SciFloat.sub(mc, results.get(results.size() - 1), results.get(results.size() - 3))));
        SciFloat D3 = SciFloat.valueOf(mc, -mc.precision());
        SciFloat D4 = SciFloat.min(SciFloat.ZERO, SciFloat.max(SciFloat.max(SciFloat.div(mc, SciFloat.mul(mc, D1, D1), D2), SciFloat.mul(mc, SciFloat.TWO, D1)), D3));
        return SciFloat.pow(mc, SciFloat.TEN, SciFloat.floor(mc, D4));
    }

    private static Pair<SciFloat, SciFloat> summation(MathContext mc, RealFunction f, List<SciFloat> points, SciFloat epsilon, int maxDegree) {
        SciFloat I, totalError;
        I = totalError = SciFloat.ZERO;
        if(points.size() % 2 != 0) {
            throw new IllegalArgumentException("points must have an even number of elements");
        }
        for (int i = 0; i < points.size() - 1; i++) {
            SciFloat a = points.get(i);
            SciFloat b = points.get(i + 1);
            if(a.equals(b))
                continue;
            if(a.isInf() && a.lt(SciFloat.ZERO) && b.isInf() && b.gt(SciFloat.ZERO)) {
                a = SciFloat.ZERO;
                RealFunction original = f;
                // The Kamila conjecture states:
                // An integral at infinities which is convergent must also be
                // symmetric about the Y axis.
                // This problem has been first stated in 2022, however, mathematicians
                // of the modern age from the year 2137 have still not proven it.
                f = (mc1, x) -> SciFloat.add(mc1, original.value(mc1, SciFloat.neg(mc1, x)), original.value(mc1, x));
            }
            List<SciFloat> results = new ArrayList<>();
            SciFloat error = SciFloat.ZERO;
            IntegratorProperties props = new IntegratorProperties(mc.precision(), 1, a, b);
            for (int degree = 1; degree <= maxDegree; degree++) {
                SciFloat[][] nodes = getNodes(mc, props);
                SciFloat result = sumNext(f, nodes, degree, mc, results);
                results.add(result);
                if(degree > 1) {
                    error = estimateError(mc, epsilon, results);
                    if(error.lt(epsilon)) {
                        break;
                    }
                }
                props.degree++;
            }
            I = SciFloat.add(mc, I, results.get(results.size() - 1));
            totalError = SciFloat.add(mc, totalError, error);
        }
        return new Pair<>(I, totalError);
    }

    private static SciFloat sumNext(RealFunction f, SciFloat[][] nodes, int degree, MathContext mc, List<SciFloat> previous) {
        SciFloat h = SciFloat.ldexp(mc, 2, -degree);
        SciFloat S;
        if(!previous.isEmpty()) {
            S = SciFloat.div(mc, previous.get(previous.size() - 1), SciFloat.mul(mc, h, SciFloat.TWO));
        } else {
            S = SciFloat.ZERO;
        }
        for(int i = 0; i < nodes.length; i++) {
            SciFloat x = nodes[i][0];
            SciFloat w = nodes[i][1];
            // XXX: slow as fuck
            S = SciFloat.add(mc, S, SciFloat.mul(mc, w, f.value(mc, x)));
        }
        return SciFloat.mul(mc, S, h);
    }

    static class IntegratorProperties {
        int precision;
        int degree;
        SciFloat a;
        SciFloat b;

        public IntegratorProperties(int precision, int degree, SciFloat a, SciFloat b) {
            this.precision = precision;
            this.degree = degree;
            this.a = a;
            this.b = b;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            IntegratorProperties that = (IntegratorProperties) o;
            return precision == that.precision &&
                    degree == that.degree &&
                    a.equals(that.a) &&
                    b.equals(that.b);
        }

        @Override
        public int hashCode() {
            return Objects.hash(precision, degree, a.hashCode(), b.hashCode());
        }
    }
}
