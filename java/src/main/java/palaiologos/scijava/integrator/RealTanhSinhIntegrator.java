package palaiologos.scijava.integrator;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.util.ConcurrentLRUCache;
import palaiologos.scijava.util.Pair;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * An implementation of the Tanh-Sinh quadrature algorithm.
 * <p> For a precise MathContext, computing the nodes and weights for the integration
 *     can be rather expensive (less so for the Tanh-Sinh quadrature though).
 *     To make repeated integrations fast, nodes are automatically cached.
 *     To free the integration nodes, use the {@link #dropCaches()} method.
 *     The cache size is limited to 256 entries and governed using the Least-Recently-Used policy.
 *
 * <p> The Tanh-Sinh quadrature tends to handle endpoint singularities very well and
 *     the nodes are cheaper to compute on the first run compared to the Gauss-Legendre quadrature.
 *
 * <p> While the Tanh-Sinh algorithm handles endpoint singularities well, it is not
 *     as good at handling interior singularities. If such occur, make sure to split the
 *     integration interval accordingly to leave out the point of singularity.
 *     The Tanh-Sinh algorithm sometimes can't cope with certain endpoint singularities
 *     (e.g. the sharp one for the square root at x=0) and will return a slightly inaccurate
 *     result. In this case, try to increase the precision temporarily.
 *
 * <p> If the desired function is smooth (infinitely differentiable), but has many sharp peaks,
 *     the precision might hinder. In these cases, consider either splitting the interval into
 *     smaller parts or increasing the degree of the quadrature.
 *
 * @author Kamila Szewczyk
 */
public final class RealTanhSinhIntegrator {
    private static final int LRU_SIZE = 256;

    private static native void getNodes(List<SciFloat[]> dest, int precision, int degree);

    private static ConcurrentLRUCache<IntegratorProperties, List<SciFloat[]>> nodeCache = new ConcurrentLRUCache<>(LRU_SIZE);

    private static List<SciFloat[]> getNodes(MathContext mc, IntegratorProperties properties) {
        List<SciFloat[]> nodes = nodeCache.get(properties);
        if (nodes == null) {
            // try to find nodes from -1 to 1 instead?
            IntegratorProperties newProperties = new IntegratorProperties(properties.precision, properties.degree, SciFloat.MINUS_ONE, SciFloat.ONE);
            List<SciFloat[]> unscaledNodes = nodeCache.get(newProperties);
            List<SciFloat[]> nodesList;
            if (unscaledNodes == null) {
                // Nope :(
                nodesList = new ArrayList<>();
                getNodes(nodesList, mc.precision() + 20, properties.degree);
            } else {
                // Found!
                nodesList = new ArrayList<>(unscaledNodes.size());
                // Deep clone
                for (SciFloat[] node : unscaledNodes)
                    nodesList.add(new SciFloat[] { node[0], node[1] });
            }
            RealIntegrator.transformNodes(mc.precision() + 20, nodesList, properties.a, properties.b);
            nodeCache.put(properties, nodesList);
            return nodesList;
        }
        return nodes;
    }

    /**
     * Free the node caches held by the Tanh-Sinh integrator.
     */
    public static void dropCaches() {
        nodeCache = new ConcurrentLRUCache<>(LRU_SIZE);
    }

    private static Pair<SciFloat, SciFloat> summation(MathContext mc, RealFunction f, List<SciFloat> points, SciFloat epsilon, int maxDegree, MathContext oldMc) {
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
                // of the modern age from the year 2137 have still not proved it.
                f = (mc1, x) -> SciFloat.add(mc1, original.value(mc1, SciFloat.neg(mc1, x)), original.value(mc1, x));
            }
            List<SciFloat> results = new ArrayList<>();
            SciFloat error = SciFloat.ZERO;
            IntegratorProperties props = new IntegratorProperties(mc.precision(), 1, a, b);
            for (int degree = 1; degree <= maxDegree; degree++) {
                List<SciFloat[]> nodes = getNodes(mc, props);
                SciFloat result = sumNext(f, nodes, degree, mc, results);
                results.add(result);
                if(degree > 1) {
                    error = RealIntegrator.estimateError(mc.precision(), epsilon, results);
                    if(error.lt(epsilon)) {
                        break;
                    }
                }
                props.degree++;
            }
            I = SciFloat.add(oldMc, I, results.get(results.size() - 1));
            totalError = SciFloat.add(mc, totalError, error);
        }
        return new Pair<>(I, totalError);
    }

    private static SciFloat sumNext(RealFunction f, List<SciFloat[]> nodes, int degree, MathContext mc, List<SciFloat> previous) {
        SciFloat h = SciFloat.ldexp(mc, 1, -degree);
        SciFloat S;
        if(!previous.isEmpty()) {
            S = SciFloat.div(mc, previous.get(previous.size() - 1), SciFloat.mul(mc, h, SciFloat.TWO));
        } else {
            S = SciFloat.ZERO;
        }
        for(int i = 0; i < nodes.size(); i++) {
            SciFloat x = nodes.get(i)[0];
            SciFloat w = nodes.get(i)[1];
            // XXX: slow as fuck
            S = SciFloat.add(mc, S, SciFloat.mul(mc, w, f.value(mc, x)));
        }
        return SciFloat.mul(mc, S, h);
    }

    /**
     * Compute the integral of the given function using the Tanh-Sinh quadrature.
     * @param mc The math context to use for the computation.
     * @param f The function to integrate.
     * @param points The endpoints to integrate at.
     * @return A pair of the approximate integral value and the error estimate.
     * @see #quad(MathContext, RealFunction, SciFloat[], int)
     */
    public static Pair<SciFloat, SciFloat> quad(MathContext mc, RealFunction f, SciFloat[] points) {
        return quad(mc, f, points, RealIntegrator.guessDegree(mc));
    }

    /**
     * Compute the integral of the given function using the Tanh-Sinh quadrature with a specified quadrature degree.
     * @param mc The math context to use for the computation.
     * @param f The function to integrate.
     * @param points The endpoints to integrate at.
     * @param max_degree The maximum degree of the quadrature.
     * @return A pair of the approximate integral value and the error estimate.
     * @see #quad(MathContext, RealFunction, SciFloat[])
     */
    public static Pair<SciFloat, SciFloat> quad(MathContext mc, RealFunction f, SciFloat[] points, int max_degree) {
        SciFloat epsilon = SciFloat.ldexp(mc, 1, 1-mc.precision());
        MathContext nmc = new MathContext(mc.precision() + 20, mc.roundingMode());
        return summation(nmc, f, Arrays.asList(points), epsilon, max_degree, mc);
    }
}
