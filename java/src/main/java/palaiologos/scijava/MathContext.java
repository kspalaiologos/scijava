package palaiologos.scijava;

public record MathContext(int precision, palaiologos.scijava.MathContext.RoundingMode roundingMode) {
    public MathContext {
        if (precision < 0) {
            throw new IllegalArgumentException("precision < 0");
        }
        if (roundingMode == null) {
            throw new IllegalArgumentException("roundingMode == null");
        }
    }

    public enum RoundingMode {
        UP,
        DOWN,
        NEAREST,
        ZERO
    }
}
