/*
    scijava
    Copyright (C) 2022 Kamila Szewczyk

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

package palaiologos.scijava;

import java.io.IOException;
import java.lang.ref.Cleaner;

import static palaiologos.scijava.NativeLibrary.load;
import static palaiologos.scijava.NativeLibrary.resourceName;

/**
 * Arbitrary precision floating point numbers provided by SciJava.
 *
 * <p> SciFloat provides methods similar to these found in {@link java.math.BigDecimal} with
 * a few firm differences. SciFloat is backed by the MPFR library, and as such the
 * internal representation of the number is not decimal, but binary. As a consequence of this,
 * the {@link MathContext} precision field represents the precision in <b>binary digits</b>,
 * not decimal digits.
 *
 * <p> SciFloat provides many methods that are not present in {@link java.math.BigDecimal}, such as
 * the gamma, digamma and Riemann Zeta functions, among others, with considerably better performance.
 *
 * @author Kamila Szewczyk
 */
public final class SciFloat implements Comparable<SciFloat>, Cloneable {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    static class CleanerRunnable implements Runnable {
        private final long pointer;

        CleanerRunnable(long pointer) {
            this.pointer = pointer;
        }

        @Override
        public void run() {
            synchronized (CleanerRunnable.class) {
                drop_caches();
            }
            SciFloat.free(pointer);
        }
    }

    final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciFloat(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, new CleanerRunnable(ptr));
    }

    private static native void free(long ptr);
    private static native String toString(long i);
    private static native void agm(int precision, int roundingMode, long dest, long a, long b);
    private static native void add(int precision, int roundingMode, long dest, long a, long b);
    private static native void sub(int precision, int roundingMode, long dest, long a, long b);
    private static native void mul(int precision, int roundingMode, long dest, long a, long b);
    private static native void div(int precision, int roundingMode, long dest, long a, long b);
    private static native void mod(int precision, int roundingMode, long dest, long a, long b);
    private static native void sqrt(int precision, int roundingMode, long dest, long a);
    private static native void sin(int precision, int roundingMode, long dest, long a);
    private static native void cos(int precision, int roundingMode, long dest, long a);
    private static native void tan(int precision, int roundingMode, long dest, long a);
    private static native void sec(int precision, int roundingMode, long dest, long a);
    private static native void csc(int precision, int roundingMode, long dest, long a);
    private static native void cot(int precision, int roundingMode, long dest, long a);
    private static native void asin(int precision, int roundingMode, long dest, long a);
    private static native void acos(int precision, int roundingMode, long dest, long a);
    private static native void atan(int precision, int roundingMode, long dest, long a);
    private static native void beta(int precision, int roundingMode, long dest, long a, long b);
    private static native void asinInplace(int precision, int roundingMode, long a);
    private static native void acosInplace(int precision, int roundingMode, long a);
    private static native void atanInplace(int precision, int roundingMode, long a);
    private static native void sinh(int precision, int roundingMode, long dest, long a);
    private static native void cosh(int precision, int roundingMode, long dest, long a);
    private static native void ai(int precision, int roundingMode, long dest, long a);
    private static native void tanh(int precision, int roundingMode, long dest, long a);
    private static native void asinh(int precision, int roundingMode, long dest, long a);
    private static native void acosh(int precision, int roundingMode, long dest, long a);
    private static native void atanh(int precision, int roundingMode, long dest, long a);
    private static native void cbrt(int precision, int roundingMode, long dest, long a);
    private static native void neg(int precision, int roundingMode, long dest, long a);
    private static native void abs(int precision, int roundingMode, long dest, long a);
    private static native void digamma(int precision, int roundingMode, long dest, long a);
    private static native void Ei(int precision, int roundingMode, long dest, long a);
    private static native void clamp(int precision, int roundingMode, long dest, long a, long min, long max);
    private static native void factorial(int precision, int roundingMode, long dest, int a);
    private static native void drop_caches();
    private static native MathContext getMathContext(long ptr);
    private static native boolean lt(long a, long b);
    private static native boolean lte(long a, long b);
    private static native boolean gt(long a, long b);
    private static native boolean gte(long a, long b);
    private static native boolean eq(long a, long b);
    private static native boolean neq(long a, long b);
    private static native int compare(long a, long b);
    private static native void copy(long dest, long src);
    private static native SciFloat fromString(int precision, int roundingMode, String s);
    private static native SciFloat fromInteger(int precision, int roundingMode, int n);
    private static native SciFloat fromSciInteger(int precision, int roundingMode, long n);
    private static native SciFloat fromSciRational(int precision, int roundingMode, long n);
    private static native SciFloat ldexp(int precision, int roundingMode, int x, int exp);
    private static native boolean isFinite(long ptr);
    private static native void ceil(int precision, int roundingMode, long dest, long a);
    private static native void pow(int precision, int roundingMode, long dest, long a, long b);
    private static native void floor(int precision, int roundingMode, long dest, long a);
    private static native void erf(int precision, int roundingMode, long dest, long a);
    private static native void exp(int precision, int roundingMode, long dest, long a);
    private static native void exp2(int precision, int roundingMode, long dest, long a);
    private static native void exp10(int precision, int roundingMode, long dest, long a);
    private static native void ln(int precision, int roundingMode, long dest, long a);
    private static native void fract(int precision, int roundingMode, long dest, long a);
    private static native void gamma(int precision, int roundingMode, long dest, long a);
    private static native void loggamma(int precision, int roundingMode, long dest, long a);
    private static native void rgamma(int precision, int roundingMode, long dest, long a);
    private static native void gammainc(int precision, int roundingMode, long dest, long a, long x);
    private static native void hypot(int precision, int roundingMode, long dest, long a, long b);
    private static native void j0(int precision, int roundingMode, long dest, long a);
    private static native void j1(int precision, int roundingMode, long dest, long a);
    private static native void jn(int precision, int roundingMode, long dest, long a, int b);

    private static native void y0(int precision, int roundingMode, long dest, long a);
    private static native void y1(int precision, int roundingMode, long dest, long a);
    private static native void yn(int precision, int roundingMode, long dest, long a, int b);
    private static native void li2(int precision, int roundingMode, long dest, long a);
    private static native void log10(int precision, int roundingMode, long dest, long a);
    private static native void radians(int precision, int roundingMode, long dest, long a);
    private static native void degrees(int precision, int roundingMode, long dest, long a);
    private static native void log2(int precision, int roundingMode, long dest, long a);
    private static native void zeta(int precision, int roundingMode, long dest, long a);
    private static native void sinpi(int precision, int roundingMode, long dest, long a);
    private static native void cospi(int precision, int roundingMode, long dest, long a);
    private static native void sinc(int precision, int roundingMode, long dest, long a);
    private static native void scale(int precision, int roundingMode, long dest, long a, int b);
    private static native void unscale(int precision, int roundingMode, long dest, long a, int b);
    private static native void recip(int precision, int roundingMode, long dest, long a);
    private static native void sech(int precision, int roundingMode, long dest, long a);
    private static native void csch(int precision, int roundingMode, long dest, long a);
    private static native void coth(int precision, int roundingMode, long dest, long a);
    private static native void harmonic(int precision, int roundingMode, long dest, long a);
    private static native void rf(int precision, int roundingMode, long dest, long x, long n);
    private static native void ff(int precision, int roundingMode, long dest, long x, long n);
    private static native void asinhInplace(int precision, int roundingMode, long a);
    private static native void acoshInplace(int precision, int roundingMode, long a);
    private static native void atanhInplace(int precision, int roundingMode, long a);
    private static native void chop(int precision, int roundingMode, long dest, long a, long eps);
    private static native SciFloat lambertw(int precision, long x, int k);
    private static native boolean isNaN(long ptr);
    private static native boolean isInf(long ptr);
    private static native SciFloat random(int precision, int roundingMode, long randptr);
    private static native SciFloat pi(int precision);
    private static native SciFloat bernoulli(int precision, int n);
    private static native SciFloat euler_gamma(int precision);
    private static native SciFloat degree(int precision);
    private static native int intValue(int precision, int roundingMode, long a);
    private static native SciFloat e(int precision);
    private static native SciFloat phi(int precision);
    private static native SciFloat catalan(int precision);
    private static native SciFloat apery(int precision);
    private static native void root(int precision, int roundingMode, long dest, long a, int n);
    private static native void log(int precision, int roundingMode, long dest, long a, long base);

    // Public API
    /**
     * The SciFloat constant 1.
     */
    public static SciFloat ONE = SciFloat.valueOf(MathContext.MC24, 1);
    /**
     * The SciFloat constant 10.
     */
    public static SciFloat TEN = SciFloat.valueOf(MathContext.MC24, 10);

    /**
     * The SciFloat constant 2.
     */
    public static SciFloat TWO = SciFloat.valueOf(MathContext.MC24, 2);

    /**
     * The SciFloat constant -1.
     */
    public static SciFloat MINUS_ONE = SciFloat.valueOf(MathContext.MC24, -1);

    /**
     * The SciFloat constant 0.
     */
    public static SciFloat ZERO = SciFloat.valueOf(MathContext.MC24, 0);

    /**
     * The SciFloat constant 0.5.
     */
    public static SciFloat HALF = SciFloat.valueOf(MathContext.MC24, "0.5");

    /**
     * The SciFloat constant representing infinity.
     */
    public static SciFloat INF = SciFloat.valueOf(MathContext.MC24, "inf");

    /**
     * The SciFloat constant representing negative infinity.
     */
    public static SciFloat NINF = SciFloat.valueOf(MathContext.MC24, "-inf");

    /**
     * Return the value of this SciFloat value as an integer.
     * @param mc the MathContext to use
     * @return the integer value of this SciFloat
     */
    public int intValue(MathContext mc) {
        return intValue(mc.precision(), mc.roundingMode().ordinal(), ptr);
    }

    /**
     * Scale a SciFloat value, that is, multiply it times a machine integral value.
     * @param mc The math context to use.
     * @param a The SciFloat value to scale.
     * @param scale The scale factor.
     * @return The scaled value.
     */
    public static SciFloat scale(MathContext mc, SciFloat a, int scale) {
        SciFloat dest = SciFloat.valueOf(mc, 0);
        scale(mc.precision(), mc.roundingMode().ordinal(), dest.ptr, a.ptr, scale);
        return dest;
    }

    /**
     * Unscale a SciFloat value, that is, divide it by a machine integral value.
     * @param mc The math context to use.
     * @param a The SciFloat value to unscale.
     * @param scale The scale factor.
     * @return The unscaled value.
     */
    public static SciFloat unscale(MathContext mc, SciFloat a, int scale) {
        SciFloat dest = SciFloat.valueOf(mc, 0);
        unscale(mc.precision(), mc.roundingMode().ordinal(), dest.ptr, a.ptr, scale);
        return dest;
    }
    
    /**
     * Return the value of the unnormalised sinc function at x.
     * @param mc The MathContext to use.
     * @param x The argument.
     * @return The value of the unnormalised sinc function at x.
     */
    public static SciFloat sinc(MathContext mc, SciFloat x) {
        if(x.eq(ZERO)) {
            return ONE;
        }
        SciFloat result = SciFloat.valueOf(mc, 0);
        sinc(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr);
        return result;
    }

    /**
     * Return the n-th harmonic number. Domain extended for fractional n, defined as euler_gamma + digamma(n + 1).
     * @param mc The MathContext to use.
     * @param n The argument.
     * @return The n-th harmonic number.
     */
    public static SciFloat harmonic(MathContext mc, SciFloat n) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        harmonic(mc.precision(), mc.roundingMode().ordinal(), result.ptr, n.ptr);
        return result;
    }

    /**
     * Return the n-th Bernoulli number.
     * @param mc The MathContext to use.
     * @param n The argument.
     * @return The n-th Bernoulli number.
     */
    public static SciFloat bernoulli(MathContext mc, int n) {
        return bernoulli(mc.precision(), n);
    }

    /**
     * Return the value of the normalised sinc function at x.
     * @param mc The MathContext to use.
     * @param x The argument.
     * @return The value of the normalised sinc function at x.
     */
    public static SciFloat sincpi(MathContext mc, SciFloat x) {
        return sinc(mc, SciFloat.mul(mc, x, SciFloat.pi(mc.precision())));
    }

    /**
     * Return the value of the reciprocal of x.
     * @param mc The MathContext to use.
     * @param x The argument.
     * @return The value of the reciprocal of x.
     */
    public static SciFloat reciprocal(MathContext mc, SciFloat x) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        recip(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr);
        return result;
    }

    /**
     * Convert a value in radians to a value in degrees.
     * @param mc The MathContext to use for the result.
     * @param a The value in radians.
     * @return The value in degrees.
     */
    public static SciFloat degrees(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        degrees(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Raise a value to the given power.
     * @param mc The MathContext to use for the result.
     * @param x The value to raise.
     * @param y The power to raise to.
     * @return The value of x raised to the power of y.
     */
    public static SciFloat pow(MathContext mc, SciFloat x, SciFloat y) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        pow(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr, y.ptr);
        return result;
    }

    /**
     * Convert a value in degrees to a value in radians.
     * @param mc The MathContext to use for the result.
     * @param a The value in degrees.
     * @return The value in radians.
     */
    public static SciFloat radians(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        radians(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of sin(pi * a)
     * @param mc The MathContext to use for the result.
     * @param a The argument.
     * @return The value of sin(pi * a)
     */
    public static SciFloat sinpi(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        sinpi(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of cos(pi * a)
     * @param mc The MathContext to use for the result.
     * @param a The argument.
     * @return The value of cos(pi * a)
     */
    public static SciFloat cospi(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        cospi(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the rising factorial (Pochhammer symbol) of x and n.
     * Defined as gamma(x+n)/gamma(x).
     * @param mc The MathContext to use for the result.
     * @param x The argument.
     * @return The value of the rising factorial of x and n.
     */
    public static SciFloat risingFactorial(MathContext mc, SciFloat x, SciFloat n) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        rf(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr, n.ptr);
        return result;
    }

    /**
     * Return the minimum (the smaller) of the two arguments.
     * @param a The first argument.
     * @param b The second argument.
     * @return The minimum of the two arguments.
     */
    public static SciFloat min(SciFloat a, SciFloat b) {
        return a.lt(b) ? a : b;
    }

    /**
     * Return the maximum (the larger) of the two arguments.
     * @param a The first argument.
     * @param b The second argument.
     * @return The maximum of the two arguments.
     */
    public static SciFloat max(SciFloat a, SciFloat b) {
        return a.gt(b) ? a : b;
    }

    /**
     * Compute the value of the beta function of x and y.
     * Defined as gamma(x) * gamma(y) / gamma(x+y).
     * @param mc The MathContext to use for the result.
     * @param x The first argument.
     * @param y The second argument.
     * @return The value of the beta function of x and y.
     */
    public static SciFloat beta(MathContext mc, SciFloat x, SciFloat y) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        beta(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr, y.ptr);
        return result;
    }

    /**
     * Compute the value of the falling factorial of x and n.
     * Defined as gamma(x+1)/gamma(x-n+1).
     * @param mc The MathContext to use for the result.
     * @param x The argument.
     * @return The value of the falling factorial of x and n.
     */
    public static SciFloat fallingFactorial(MathContext mc, SciFloat x, SciFloat n) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        ff(mc.precision(), mc.roundingMode().ordinal(), result.ptr, x.ptr, n.ptr);
        return result;
    }

    /**
     * Return the value of the Lambert W function (branch k) of x.
     * @param mc The MathContext to use for the result.
     * @param x The argument of the Lambert W function.
     * @param k The branch of the Lambert W function.
     * @return The value of the Lambert W function of x.
     */
    public static SciFloat lambertw(MathContext mc, SciFloat x, int k) {
        return lambertw(mc.precision(), x.ptr, k);
    }

    /**
     * Compute the logarithm in a given base of a SciFloat value.
     * @param mc The MathContext to use for the computation.
     * @param a The value to compute the logarithm of.
     * @param base The base of the logarithm.
     * @return The logarithm of a in selected base.
     * @throws IllegalArgumentException when base <= 0
     */
    public static SciFloat log(MathContext mc, SciFloat a, SciFloat base) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        log(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, base.ptr);
        return result;
    }

    /**
     * Computes the n-th root of a SciFloat.
     * @param mc The MathContext to use.
     * @param a The SciFloat to compute the root of.
     * @param n The degree of the root to compute.
     * @return The n-th root of a.
     * @throws IllegalArgumentException when n <= 0.
     */
    public static SciFloat root(MathContext mc, SciFloat a, int n) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        root(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, n);
        return result;
    }

    /**
     * Return the value of the constant pi with the given precision.
     *
     * @param mc The precision of the constant.
     * @return The value of pi.
     */
    public static SciFloat pi(MathContext mc) {
        return pi(mc.precision());
    }

    /**
     * Return the value of one degree in radians.
     *
     * @param mc The precision of the constant.
     * @return The value of 1 degree.
     */
    public static SciFloat degree(MathContext mc) {
        return degree(mc.precision());
    }

    /**
     * Return the value of the constant that is the base of the natural logarithm.
     *
     * @param mc The precision of the constant.
     * @return The value of e.
     */
    public static SciFloat e(MathContext mc) {
        return e(mc.precision());
    }

    /**
     * Return the value of the Golden ratio.
     *
     * @param mc The precision of the constant.
     * @return The value of phi.
     */
    public static SciFloat phi(MathContext mc) {
        return phi(mc.precision());
    }

    /**
     * Return the value of the Catalan constant.
     *
     * @param mc The precision of the constant.
     * @return The value of the Catalan's constant.
     */
    public static SciFloat catalan(MathContext mc) {
        return catalan(mc.precision());
    }

    /**
     * Return the value of the Apery's constant.
     *
     * @param mc The precision of the constant.
     * @return The value of Apery's constant.
     */
    public static SciFloat apery(MathContext mc) {
        return apery(mc.precision());
    }

    /**
     * Return the value of the Euler's gamma constant (approx. 0.577) with the given precision.
     *
     * @param mc The precision of the constant.
     * @return The value of Euler's gamma constant.
     */
    public static SciFloat eulerGamma(MathContext mc) {
        return euler_gamma(mc.precision());
    }

    /**
     * Generate a random number in range 0 <= x < 1.
     * @param mc the math context to use for the operation.
     * @param random the random number generator instance to be used for generating
     *               the resulting value.
     * @return A random number in range 0 <= x < 1.
     */
    public static SciFloat random(MathContext mc, Random random) {
        return random(mc.precision(), mc.roundingMode().hashCode(), random.ptr);
    }

    /**
     * Determine if the SciFloat is NaN.
     * @return true if the SciFloat is NaN, false otherwise.
     */
    public boolean isNaN() {
        return isNaN(ptr);
    }

    /**
     * Determine if the SciFloat is infinite.
     * @return true if the SciFloat is infinite, false otherwise.
     */
    public boolean isInf() {
        return isInf(ptr);
    }

    /**
     * Converts numbers close to another numbers to that number.
     * For example, chop(1e-10, 1e-8) = 0, chop(0.99999999999923, 1e-10) = 1
     * @param mc The math context to use during the operation.
     * @param a The number to chop.
     * @param eps The epsilon value.
     * @return The chopped number.
     */
    public static SciFloat chop(MathContext mc, SciFloat a, SciFloat eps) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        chop(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, eps.ptr);
        return result;
    }

    /**
     * The error function.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The value of the error function for argument a.
     */
    public static SciFloat erf(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        erf(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * The exponential function. Defined as e^a, where e is the Napier constant,
     * commonly known as the base of the natural logarithm.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The value of exp(a).
     */
    public static SciFloat exp(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        exp(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * The base-2 exponential function. Defined as 2^a.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The value of exp2(a).
     */
    public static SciFloat exp2(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        exp2(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * The base-10 exponential function. Defined as 10^a.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The value of exp10(a).
     */
    public static SciFloat exp10(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        exp10(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * The natural logarithm. Often denoted as ln(a) to avoid confusion with
     * the base-10 or base-2 logarithm.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The value of ln(a).
     */
    public static SciFloat ln(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        ln(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Extract the fractional part of a number, can be thought of as {@code x - floor(x)} for {@code x >= 0}
     * and {@code x - ceil(x)} for {@code x < 0}.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The fractional part of a.
     */
    public static SciFloat fract(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        fract(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Round up a number to the next higher integer.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The smallest integer greater than or equal to a.
     */
    public static SciFloat ceil(MathContext mc, SciFloat a) {
        var result = SciFloat.valueOf(mc, 0);
        SciFloat.ceil(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Round down a number to the next lower integer.
     *
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return The largest integer less than or equal to a.
     */
    public static SciFloat floor(MathContext mc, SciFloat a) {
        var result = SciFloat.valueOf(mc, 0);
        SciFloat.floor(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Convert an integer value to a SciFloat.
     *
     * @param mc The math context to use for the resulting SciFloat.
     * @param n The integer value to convert.
     * @return The SciFloat representation of n.
     */
    public static SciFloat valueOf(MathContext mc, int n) {
        return fromInteger(mc.precision(), mc.roundingMode().ordinal(), n);
    }

    /**
     * Parse a string value to a SciFloat.
     *
     * @param mc The math context to use for the resulting SciFloat.
     * @param s The string value to parse.
     * @return The SciFloat representation of s.
     * @throws NumberFormatException If the string is not a valid representation of a SciFloat.
     */
    public static SciFloat valueOf(MathContext mc, String s) {
        return fromString(mc.precision(), mc.roundingMode().ordinal(), s);
    }

    /**
     * Convert a SciInteger value to a SciFloat.
     *
     * @param mc The math context to use for the resulting SciFloat.
     * @param i The SciInteger value to convert.
     * @return The SciFloat representation of i.
     */
    public static SciFloat valueOf(MathContext mc, SciInteger i) {
        return fromSciInteger(mc.precision(), mc.roundingMode().ordinal(), i.ptr);
    }

    /**
     * Convert a SciInteger value to a SciFloat.
     *
     * @param mc The math context to use for the resulting SciFloat.
     * @param i The SciInteger value to convert.
     * @return The SciFloat representation of i.
     */
    public static SciFloat valueOf(MathContext mc, SciRational i) {
        return fromSciRational(mc.precision(), mc.roundingMode().ordinal(), i.ptr);
    }

    /**
     * Compute the value of the arithmetic-geometric mean.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return The value of the arithmetic-geometric mean of a and b.
     */
    public static SciFloat agm(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        agm(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the sum of two SciFloat values.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return a + b.
     */
    public static SciFloat add(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.add(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the difference of two SciFloat values.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return a - b.
     */
    public static SciFloat sub(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sub(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the product of two SciFloat values.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return a * b.
     */
    public static SciFloat mul(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.mul(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the quotient of two SciFloat values.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return a / b.
     */
    public static SciFloat div(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.div(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the modulus of two SciFloat values.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return a % b.
     */
    public static SciFloat mod(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.mod(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the square root of a SciFloat value.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return sqrt(a).
     */
    public static SciFloat sqrt(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sqrt(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function sin(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return sin(a).
     */
    public static SciFloat sin(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sin(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function cos(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return cos(a).
     */
    public static SciFloat cos(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cos(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function tan(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return tan(a).
     */
    public static SciFloat tan(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.tan(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function sec(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return sec(a).
     */
    public static SciFloat sec(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sec(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function csc(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return csc(a).
     */
    public static SciFloat csc(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.csc(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the trigonometric function cot(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return cot(a).
     */
    public static SciFloat cot(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cot(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arcsin(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arcsin(a).
     */
    public static SciFloat asin(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.asin(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arccos(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arccos(a).
     */
    public static SciFloat acos(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.acos(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arctan(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arctan(a).
     */
    public static SciFloat atan(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.atan(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arcsec(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arcsec(a).
     */
    public static SciFloat asec(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.div(mc.precision(), mc.roundingMode().ordinal(), result.ptr, ONE.ptr, a.ptr);
        SciFloat.acosInplace(mc.precision(), mc.roundingMode().ordinal(), result.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arccsc(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arccsc(a).
     */
    public static SciFloat acsc(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.div(mc.precision(), mc.roundingMode().ordinal(), result.ptr, ONE.ptr, a.ptr);
        SciFloat.asinInplace(mc.precision(), mc.roundingMode().ordinal(), result.ptr);
        return result;
    }

    /**
     * Compute the value of the cyclometric function arccot(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return arccot(a).
     */
    public static SciFloat acot(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.div(mc.precision(), mc.roundingMode().ordinal(), result.ptr, ONE.ptr, a.ptr);
        SciFloat.atanInplace(mc.precision(), mc.roundingMode().ordinal(), result.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function sinh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return sinh(a).
     */
    public static SciFloat sinh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sinh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function sech(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return sech(a).
     */
    public static SciFloat sech(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sech(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function csch(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return csch(a).
     */
    public static SciFloat csch(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.csch(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function coth(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return coth(a).
     */
    public static SciFloat coth(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.coth(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function cosh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return cosh(a).
     */
    public static SciFloat cosh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cosh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the Airy function Ai(a) a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return Ai(a).
     */
    public static SciFloat ai(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.ai(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the hyperbolic function tanh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return tanh(a).
     */
    public static SciFloat tanh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.tanh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the inverse hyperbolic function asinh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return asinh(a).
     */
    public static SciFloat asinh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.asinh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the inverse hyperbolic function acosh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return acosh(a).
     */
    public static SciFloat acosh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.acosh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the inverse hyperbolic function atanh(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return atanh(a).
     */
    public static SciFloat atanh(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.atanh(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the inverse hyperbolic function asech(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return asech(a).
     */
    public static SciFloat asech(MathContext mc, SciFloat a) {
        SciFloat x = SciFloat.reciprocal(mc, a);
        acoshInplace(mc.precision(), mc.roundingMode().ordinal(), x.ptr);
        return x;
    }

    /**
     * Compute the value of the inverse hyperbolic function acsch(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return acsch(a).
     */
    public static SciFloat acsch(MathContext mc, SciFloat a) {
        SciFloat x = SciFloat.reciprocal(mc, a);
        asinhInplace(mc.precision(), mc.roundingMode().ordinal(), x.ptr);
        return x;
    }

    /**
     * Compute the value of the inverse hyperbolic function acoth(a) of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return acoth(a).
     */
    public static SciFloat acoth(MathContext mc, SciFloat a) {
        SciFloat x = SciFloat.reciprocal(mc, a);
        atanhInplace(mc.precision(), mc.roundingMode().ordinal(), x.ptr);
        return x;
    }

    /**
     * Compute the cube root of a SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The cube root of a.
     */
    public static SciFloat cbrt(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cbrt(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the digamma function of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The digamma function of a.
     */
    public static SciFloat digamma(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.digamma(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the ln(gamma(x)) function of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The logarithm of the gamma function function of a.
     */
    public static SciFloat loggamma(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.loggamma(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the exponential integral of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The exponential integral of a.
     */
    public static SciFloat Ei(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.Ei(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Negate a SciFloat value. Does not modify the operand.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return -a.
     */
    public static SciFloat neg(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.neg(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Yield the absolute value of a SciFloat value. Does not modify the operand.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return abs(a).
     */
    public static SciFloat abs(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.abs(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the gamma function of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The gamma function of a.
     */
    public static SciFloat gamma(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.gamma(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the inverse gamma (1 / gamma(x)) function of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The inverse gamma function of a.
     */
    public static SciFloat rgamma(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.rgamma(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the upper incomplete gamma function of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param x The second argument.
     * @return The upper incomplete gamma function of a and x.
     */
    public static SciFloat igamma(MathContext mc, SciFloat a, SciFloat x) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.gammainc(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, x.ptr);
        return result;
    }

    /**
     * Compute the value of the factorial of an integer as a SciFloat value.
     * It might be more useful to consider the {@link SciInteger} factorial method
     * or the gamma functon from this class.
     *
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @return The factorial of a.
     */
    public static SciFloat factorial(MathContext mc, int a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.factorial(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a);
        return result;
    }

    /**
     * Clamp a SciFloat value to a given range.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param min The minimum value.
     * @param max The maximum value.
     * @return The clamped value.
     * @throws IllegalArgumentException if min > max.
     */
    public static SciFloat clamp(MathContext mc, SciFloat a, SciFloat min, SciFloat max) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.clamp(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, min.ptr, max.ptr);
        return result;
    }

    /**
     * Compute the euclidean norm of two SciFloat values.
     * @param mc The math context to use while performing computations.
     * @param a The first argument.
     * @param b The second argument.
     * @return sqrt(a^2 + b^2).
     */
    public static SciFloat hypot(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.hypot(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Load exponent, that is, compute the value of a * 2^exp.
     * @param mc The math context to use for the resulting SciFlaot value.
     * @param a The first argument.
     * @param b The second argument.
     * @return a * 2^exp.
     */
    public static SciFloat ldexp(MathContext mc, int a, int b) {
        return ldexp(mc.precision(), mc.roundingMode().ordinal(), a, b);
    }

    /**
     * Check whether a SciFloat value is finite, that is, not NaN or infinity.
     * @return true if the value is finite, false otherwise.
     */
    public boolean isFinite() {
        return isFinite(ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when the values are equal, false otherwise.
     */
    public boolean eq(SciFloat other) {
        return eq(ptr, other.ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when the values are not equal, false otherwise.
     */
    public boolean neq(SciFloat other) {
        return neq(ptr, other.ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when this value is less than the other value, false otherwise.
     */
    public boolean lt(SciFloat other) {
        return lt(ptr, other.ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when this value is less than or equal to the other value, false otherwise.
     */
    public boolean lte(SciFloat other) {
        return lte(ptr, other.ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when this value is greater than the other value, false otherwise.
     */
    public boolean gt(SciFloat other) {
        return gt(ptr, other.ptr);
    }

    /**
     * Compare this SciFloat value to another SciFloat value.
     * @param other The other SciFloat value.
     * @return true when this value is greater than or equal to the other value, false otherwise.
     */
    public boolean gte(SciFloat other) {
        return gte(ptr, other.ptr);
    }

    /**
     * Determine the equality of this SciFloat value to another SciFloat value.
     * The advantage of using this method over the {@link #eq(SciFloat)} method in
     * some scenarios is that this method can detect physical equality of the
     * underlying MPFR pointers, giving a slight performance improvement in some
     * cases.
     *
     * @param other The other SciFloat value.
     * @return true when the values are equal, false otherwise.
     */
    @Override
    public boolean equals(Object other) {
        if (other == this) {
            return true;
        }

        if (other instanceof SciFloat) {
            return eq((SciFloat) other);
        }

        return false;
    }

    /**
     * Stringify this SciFloat value.
     * @return The string representation of this SciFloat value.
     */
    @Override
    public String toString() {
        return toString(ptr);
    }

    /**
     * Return the value of performing three-way comparison between this SciFloat value and another SciFloat value.
     * @param o the object to be compared.
     * @return a negative integer when {@code this < other}, zero when {@code this = other}, or a positive integer when {@code this > other}.
     */
    @Override
    public int compareTo(SciFloat o) {
        return compare(ptr, o.ptr);
    }

    /**
     * Clone this SciFloat value.
     * @return A deep copy of this SciFloat value.
     */
    @Override
    protected Object clone() {
        SciFloat result = SciFloat.valueOf(getMathContext(ptr), 0);
        copy(result.ptr, ptr);
        return result;
    }

    /**
     * Get the hash code of this SciFloat value.
     * Relatively slow: yields the hash code of the string representation of this SciFloat value.
     * @return The hash code of this SciFloat value.
     */
    @Override
    public int hashCode() {
        return toString().hashCode();
    }

    /**
     * Return the value of the Bessel function of order zero.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return bessel_j0(a).
     */
    public static SciFloat j0(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.j0(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the Bessel function of order one.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return bessel_j1(a).
     */
    public static SciFloat j1(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.j1(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the Bessel function of order n.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @param n The order.
     * @return bessel_jn(a).
     */
    public static SciFloat jn(MathContext mc, int n, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.jn(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, n);
        return result;
    }

    /**
     * Return the value of the Bessel function of order zero.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return bessel_y0(a).
     */
    public static SciFloat y0(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.y0(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the Bessel function of order one.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return bessel_j1(a).
     */
    public static SciFloat y1(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.y1(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Return the value of the Bessel function of order n.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @param n The order.
     * @return bessel_jn(a).
     */
    public static SciFloat yn(MathContext mc, int n, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.yn(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, n);
        return result;
    }

    /**
     * Compute the value of the dilogarithm of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return Li2(a)
     */
    public static SciFloat li2(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.li2(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the base-2 logarithm of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return log2(a)
     */
    public static SciFloat log2(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.log2(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the Riemann Zeta of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return zeta(a)
     */
    public static SciFloat zeta(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.zeta(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the base-10 logarithm of a given SciFloat value.
     * @param mc The math context to use while performing computations.
     * @param a The argument.
     * @return log10(a)
     */
    public static SciFloat log10(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.log10(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }
}
