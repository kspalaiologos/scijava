package palaiologos.scijava;

import java.util.List;

public class TanhSinhIntegrator {
    private int guessDegree(MathContext mc) {
        return 6 + Math.max(0, (int) Math.ceil(Math.log(mc.precision() / 30.0) / Math.log(2)));
    }

    private static native SciFloat[][] transformNodes(int precision, SciFloat[][] orig, SciFloat a, SciFloat b);
    private static native SciFloat[][] getNodes(int precision, int degree);
}
