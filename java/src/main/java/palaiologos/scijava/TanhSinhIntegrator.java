package palaiologos.scijava;

import java.util.List;

public class TanhSinhIntegrator {
    private int guessDegree(MathContext mc) {
        return 6 + Math.max(0, (int) Math.ceil(Math.log(mc.precision() / 30.0) / Math.log(2)));
    }

    private static native List<SciFloat[]> getNodes(int precision, int degree);
}
