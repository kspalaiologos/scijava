package palaiologos.scijava.integrator;

import palaiologos.scijava.MathContext;
import palaiologos.scijava.SciFloat;

import java.util.List;

class RealIntegrator {
    public static int guessDegree(MathContext mc) {
        return 6 + Math.max(0, (int) Math.ceil(Math.log(mc.precision() / 30.0) / Math.log(2)));
    }

    public static native void transformNodes(int precision, List<SciFloat[]> orig, SciFloat a, SciFloat b);

    public static native SciFloat estimateError(int precision, SciFloat epsilon, List<SciFloat> results);
}
