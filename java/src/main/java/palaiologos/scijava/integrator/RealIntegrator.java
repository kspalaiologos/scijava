package palaiologos.scijava.integrator;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;

import java.util.List;

class RealIntegrator {
    public static int guessDegree(MathContext mc) {
        return 6 + Math.max(0, (int) Math.ceil(Math.log(mc.precision() / 30.0) / Math.log(2)));
    }

    public static native void transformNodes(int precision, List<SciFloat[]> orig, SciFloat a, SciFloat b);

    public static SciFloat estimateError(MathContext mc, SciFloat epsilon, List<SciFloat> results) {
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
}
