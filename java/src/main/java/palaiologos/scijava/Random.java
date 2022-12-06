package palaiologos.scijava;

import java.io.IOException;
import java.lang.ref.Cleaner;

import static palaiologos.scijava.NativeLibrary.load;
import static palaiologos.scijava.NativeLibrary.resourceName;

/**
 * Facilities for random number generation in SciJava using the
 * Mersenne Twister algorithm.
 *
 * @author Kamila Szewczyk
 */
public final class Random {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    final long ptr;

    private final Cleaner.Cleanable cleanable;

    /**
     * Creates a new random number generator instance.
     */
    public Random() {
        long ptr = newMersenneTwister();
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, () -> {
            Random.free(ptr);
        });
    }

    private static native void free(long ptr);
    private static native long newMersenneTwister();

    private static native long range(long ptr, int max);
    private static native void seed(long ptr, long src);

    // Public API:
    /**
     * Return a random number in the range [0, max).
     * @param max The upper bound.
     * @return a random number in the range [0, max).
     * @throws IllegalArgumentException if max is negative or zero.
     */
    public long range(int max) {
        return range(ptr, max);
    }

    /**
     * Seed the Mersenne Twister instance with a {@code SciInteger} value.
     * @param seed The seed value.
     */
    public void seed(SciInteger seed) {
        seed(ptr, seed.ptr);
    }
}
