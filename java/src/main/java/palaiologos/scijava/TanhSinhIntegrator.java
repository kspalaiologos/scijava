package palaiologos.scijava;

import palaiologos.scijava.util.ConcurrentLRUCache;

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

    private SciFloat[][] getNodes(MathContext mc, IntegratorProperties properties) {
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
            // XXX: slow like fuck
            S = SciFloat.add(mc, S, SciFloat.mul(mc, w, f.value(mc, x)));
        }
        return SciFloat.mul(mc, S, h);
    }

    class IntegratorProperties {
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
