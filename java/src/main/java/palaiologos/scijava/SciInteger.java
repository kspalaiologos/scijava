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
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

import static palaiologos.scijava.NativeLibrary.load;
import static palaiologos.scijava.NativeLibrary.resourceName;

/**
 * Arbitrary precision integers provided by SciJava.
 *
 * <p>SciInteger provides methods that are similar to those in {@link java.math.BigInteger}, such
 * as basic arithmetics, comparison, GCD/LCM, factorials, bit operations, primality-related utilities,
 * fibonacci numbers, lucas numbers and integer factorisation using the Pollard-Rho algorithm.
 *
 * <p>SciInteger is backed by the GMP library.
 *
 * @author Kamila Szewczyk
 */
public final class SciInteger implements Comparable<SciInteger>, Cloneable {
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
            SciInteger.free(pointer);
        }
    }

    final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciInteger(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, new CleanerRunnable(ptr));
    }

    private static native void free(long ptr);
    
    private static native String toString(long i);
    private static native String toStringRadix(long i, int radix);

    private static native void add(long dest, long a, long b);
    private static native void sub(long dest, long a, long b);
    private static native void mul(long dest, long a, long b);
    private static native void div(long dest, long a, long b);
    private static native void rem(long dest, long a, long b);
    private static native void pow(long dest, long a, int b);
    private static native void negate(long dest, long a);
    private static native void abs(long dest, long a);
    private static native void gcd(long dest, long a, long b);
    private static native void lcm(long dest, long a, long b);
    private static native void factorial(long dest, int a);
    private static native void signum(long dest, long a);
    private static native boolean lt(long a, long b);
    private static native boolean lte(long a, long b);
    private static native boolean gt(long a, long b);
    private static native boolean gte(long a, long b);
    private static native boolean eq(long a, long b);
    private static native boolean neq(long a, long b);
    private static native int compare(long a, long b);
    private static native void and(long dest, long a, long b);
    private static native void or(long dest, long a, long b);
    private static native void xor(long dest, long a, long b);
    private static native void not(long dest, long a);
    private static native void shl(long dest, long a, int b);
    private static native void shr(long dest, long a, int b);
    private static native void setBit(long dest, long a, int b);
    private static native void clearBit(long dest, long a, int b);
    private static native void flipBit(long dest, long a, int b);
    private static native boolean testBit(long a, int b);
    private static native int bitCount(long a);
    private static native int bitLength(long a);
    private static native boolean isPrime(long a, int certainty);
    private static native void nextPrime(long dest, long a);
    private static native void clamp(long dest, long a, long min, long max);
    private static native void divmod(long destdiv, long destmod, long a, long b);
    private static native void fibonacci(long dest, int a);
    private static native void lucas(long dest, int a);
    private static native int hamming(long a, long b);
    private static native void sqrt(long dest, long a);
    private static native void binomial(long dest, long a, int k);
    private static native void square(long dest, long a);
    private static native int jacobi(long a, long b);
    private static native int legendre(long a, long b);
    private static native SciInteger fromInteger(int i);
    private static native SciInteger fromSciFloat(long i);
    private static native SciInteger fromString(String s);
    private static native SciInteger fromStringRadix(String s, int radix);
    private static native int toInteger(long i);
    private static native void copy(long dest, long src);
    private static native void factor(Map<SciInteger, SciInteger> destFactors, long a);
    private static native SciInteger randomBits(long rptr, int bits);
    private static native SciInteger randomRange(long rptr, long max);

    /**
     * The SciInteger constant 0.
     */
    public static final SciInteger ZERO = fromInteger(0);

    /**
     * The SciInteger constant 1.
     */
    public static final SciInteger ONE = fromInteger(1);

    /**
     * The SciInteger constant 2.
     */
    public static final SciInteger TWO = fromInteger(2);

    /**
     * The SciInteger constant 5.
     */
    public static final SciInteger FIVE = fromInteger(5);

    /**
     * The SciInteger constant 10.
     */
    public static final SciInteger TEN = fromInteger(10);

    /**
     * Generate a SciInteger with a random value that has N bits.
     * @param random the random number generator to use
     * @param bits the number of bits
     * @return a random SciInteger
     * @throws NullPointerException if random is null
     * @throws IllegalArgumentException if bits is negative
     */
    public static SciInteger randomBits(Random random, int bits) {
        return randomBits(random.ptr, bits);
    }

    /**
     * Generate a SciInteger with a random value that is less than max.
     * @param random random number generator instance
     * @param max maximum value
     * @return random SciInteger
     * @throws NullPointerException if random or max is null
     * @throws IllegalArgumentException if max is negative
     */
    public static SciInteger randomRange(Random random, SciInteger max) {
        return randomRange(random.ptr, max.ptr);
    }

    /**
     * Return a new SciInteger with the value of the specified integer.
     * @param i the integer to convert
     * @return a new SciInteger instance
     */
    public static SciInteger valueOf(int i) {
        return fromInteger(i);
    }

    /**
     * Return a new SciInteger with the value of the specified string.
     * @param s the integer to convert
     * @return a new SciInteger instance
     * @throws ArithmeticException if the string does not contain a valid integer.
     */
    public static SciInteger valueOf(String s) {
        return fromString(s);
    }

    /**
     * Return a new SciInteger with the value of the specified string.
     * @param s the integer to convert
     * @param radix the radix to use (2-36)
     * @return a new SciInteger instance
     * @throws ArithmeticException if the string does not contain a valid integer or radix is outside of range between 2 and 36.
     */
    public static SciInteger valueOf(String s, int radix) {
        return fromStringRadix(s, radix);
    }

    /**
     * Return a new SciInteger with the value of the specified SciFloat.
     * @param f the SciFloat to convert
     * @return a new SciInteger instance
     * @throws ArithmeticException if the SciFloat is not finite.
     */
    public static SciInteger valueOf(SciFloat f) {
        return fromSciFloat(f.ptr);
    }

    /**
     * Add two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a + b
     */
    public static SciInteger add(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        add(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Subtract two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a - b
     */
    public static SciInteger subtract(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        sub(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Multiply two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a * b
     */
    public static SciInteger multiply(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        mul(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Divide two SciIntegers to produce a new SciInteger instance containing the quotient.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a / b
     * @throws ArithmeticException if b is zero
     */
    public static SciInteger divide(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        div(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Divide two SciIntegers to produce a new SciInteger instance containing the remainder.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a % b
     * @throws ArithmeticException if b is zero
     */
    public static SciInteger mod(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        rem(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Raise a SciInteger to a power to produce a new SciInteger instance.
     * Does not modify the operand.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of {@code a ^ b}
     * @throws ArithmeticException if {@code b < 0}
     */
    public static SciInteger pow(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        pow(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Negate a SciInteger to produce a new SciInteger instance.
     * Does not modify the operand.
     * @param a the operand
     * @return a new SciInteger instance, the result of -a
     */
    public static SciInteger negate(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        negate(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the absolute value of a SciInteger to produce a new SciInteger instance.
     * Does not modify the operand.
     * @param a the operand
     * @return a new SciInteger instance, the result of |a|
     */
    public static SciInteger abs(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        abs(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the greatest common divisor of two SciIntegers to produce a new SciInteger instance.
     * Arguments may be negative or zero. Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of gcd(a, b)
     */
    public static SciInteger gcd(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        gcd(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the least common multiple of two SciIntegers to produce a new SciInteger instance.
     * Arguments may be negative or zero. Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of lcm(a, b)
     */
    public static SciInteger lcm(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        lcm(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the factorial of a natural number to produce a new SciInteger instance.
     * @param a the operand
     * @return a new SciInteger instance, the result of a!
     * @throws ArithmeticException if {@code a < 0}
     */
    public static SciInteger factorial(int a) {
        SciInteger result = SciInteger.fromInteger(0);
        factorial(result.ptr, a);
        return result;
    }

    /**
     * Compute the sign of a SciInteger.
     * @param a the operand
     * @return -1 if {@code a < 0}, 0 if {@code a = 0}, 1 if {@code a > 0}
     */
    public static SciInteger signum(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        signum(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * @param b the other SciInteger
     * @return true if this SciInteger is less than b, false otherwise
     */
    public boolean lt(SciInteger b) {
        return lt(ptr, b.ptr);
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * @param b the other SciInteger
     * @return true if this SciInteger is less than or equal to b, false otherwise
     */
    public boolean lte(SciInteger b) {
        return lte(ptr, b.ptr);
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * @param b the other SciInteger
     * @return true if this SciInteger is greater than b, false otherwise
     */
    public boolean gt(SciInteger b) {
        return gt(ptr, b.ptr);
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * @param b the other SciInteger
     * @return true if this SciInteger is greater than or equal to b, false otherwise
     */
    public boolean gte(SciInteger b) {
        return gte(ptr, b.ptr);
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * Notice that this method always involves the underlying equality checks. When dealing
     * with objects of unknown origin, consider using the {@link #equals(Object)} method first
     * to avoid unnecessary equality checks in case the objects are physically equal or the
     * types differ.
     * @param b the other SciInteger
     * @return true if this SciInteger is equal to b, false otherwise
     */
    public boolean eq(SciInteger b) {
        return eq(ptr, b.ptr);
    }

    /**
     * Compare this SciInteger with another SciInteger to determine their relative order.
     * Notice that this method always involves the underlying equality checks. When dealing
     * with objects of unknown origin, consider using the {@link #equals(Object)} method first
     * to avoid unnecessary equality checks in case the objects are physically equal or the
     * types differ.
     * @param b the other SciInteger
     * @return true if this SciInteger is not equal to b, false otherwise
     */
    public boolean neq(SciInteger b) {
        return neq(ptr, b.ptr);
    }

    /**
     * Compute the value of the bit-wise "and" function of two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of {@code a & b}
     */
    public static SciInteger and(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        and(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the value of the bit-wise "or" function of two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a | b
     */
    public static SciInteger or(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        or(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the value of the bit-wise "xor" function of two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a ^ b
     */
    public static SciInteger xor(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        xor(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Compute the value of the bit-wise "not" function of a SciInteger to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the operand
     * @return a new SciInteger instance, the result of ~a
     */
    public static SciInteger not(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        not(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the bit-wise "shift left" function of two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of {@code a << b}
     */
    public static SciInteger shl(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        shl(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Compute the value of the bit-wise "shift right" function of two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of {@code a >> b}
     */
    public static SciInteger shr(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        shr(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Set the specified bit of a SciInteger to 1. Does not modify the operand.
     * @param a the operand
     * @param b the bit index
     * @return a new SciInteger instance, the result of {@code a | (1 << b)}
     * @throws ArithmeticException if b is negative
     */
    public static SciInteger setBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        setBit(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Set the specified bit of a SciInteger to 0. Does not modify the operand.
     * @param a the operand
     * @param b the bit index
     * @return a new SciInteger instance, the result of {@code a & ~(1 << b)}
     * @throws ArithmeticException if b is negative
     */
    public static SciInteger clearBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        clearBit(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Compute the value of the bit at the specified index of a SciInteger. Does not modify the operand.
     * @param a the operand
     * @param b the bit index
     * @return the value of the bit at index b of a
     * @throws ArithmeticException if b is negative
     */
    public static boolean testBit(SciInteger a, int b) {
        return testBit(a.ptr, b);
    }

    /**
     * Flip the value of the bit at the specified index of a SciInteger. Does not modify the operand.
     * @param a the operand
     * @param b the bit index
     * @return a new SciInteger instance, the result of {@code a ^ (1 << b)}
     * @throws ArithmeticException if b is negative
     */
    public static SciInteger flipBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        flipBit(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Compute the number of bits in the binary representation of a SciInteger. Does not modify the operand.
     * @param a the operand
     * @return the number of bits in the binary representation of a
     */
    public static int bitLength(SciInteger a) {
        return bitLength(a.ptr);
    }

    /**
     * Compute the number of set bits in the binary representation of a SciInteger. Does not modify the operand.
     * Because of two complement representation, the number of set bits is equal to the number of zero bits iff
     * a is negative and the number of one bits iff a is positive.
     * @param a the operand
     * @return the number of set bits in the binary representation of a
     */
    public static int bitCount(SciInteger a) {
        return bitCount(a.ptr);
    }

    /**
     * Determine whether a SciInteger is a prime number. Does not modify the operand.
     * Does not provide a guarantee of primality, a probabilistic test is used.
     * @param a the operand
     * @param certainty the number of primality-checking rounds to perform
     * @return true if a is likely prime, false otherwise
     * @throws ArithmeticException if certainty is negative
     */
    public static boolean isPrime(SciInteger a, int certainty) {
        return isPrime(a.ptr, certainty);
    }

    /**
     * Find the next prime number after a SciInteger. Does not modify the operand.
     * @param a the operand
     * @return a new SciInteger instance, the next prime number after a
     */
    public static SciInteger nextPrime(SciInteger a) {
        SciInteger result = SciInteger.valueOf(0);
        nextPrime(result.ptr, a.ptr);
        return result;
    }

    /**
     * Divide two SciIntegers and yield both the quotient and the remainder. Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return an array of SciInteger[] instances, the results of (respectively) a / b and a % b
     */
    public static SciInteger[] divMod(SciInteger a, SciInteger b) {
        SciInteger[] tab = new SciInteger[2];
        tab[0] = SciInteger.fromInteger(0);
        tab[1] = SciInteger.fromInteger(0);
        divmod(tab[0].ptr, tab[1].ptr, a.ptr, b.ptr);
        return tab;
    }

    /**
     * Compute the value of the Binomial coefficient of a SciInteger and integer to produce a new SciInteger instance.
     * Does not modify the operand.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a choose b
     */
    public static SciInteger binomial(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        binomial(result.ptr, a.ptr, b);
        return result;
    }

    /**
     * Compute the n-th Fibonacci number to produce a new SciInteger instance.
     * @param n the index of the Fibonacci number to compute
     * @return a new SciInteger instance, the n-th Fibonacci number
     * @throws ArithmeticException if n is negative
     */
    public static SciInteger fibonacci(int n) {
        SciInteger result = SciInteger.fromInteger(0);
        fibonacci(result.ptr, n);
        return result;
    }

    /**
     * Clamp the value of a SciInteger to the range [min, max] to produce a new SciInteger instance.
     * @param a the operand
     * @param min the lower bound
     * @param max the upper bound
     * @return a new SciInteger instance, the result of min(max(a, min), max)
     * @throws ArithmeticException if min > max
     */
    public static SciInteger clamp(SciInteger a, SciInteger min, SciInteger max) {
        SciInteger result = SciInteger.fromInteger(0);
        clamp(result.ptr, a.ptr, min.ptr, max.ptr);
        return result;
    }

    /**
     * Return the n-th Lucas number.
     * @param n the index of the Lucas number to compute
     * @return a new SciInteger instance, the n-th Lucas number
     * @throws ArithmeticException if n is negative
     */
    public static SciInteger lucas(int n) {
        SciInteger result = SciInteger.fromInteger(0);
        lucas(result.ptr, n);
        return result;
    }

    /**
     * Compute the Hamming distance between two SciIntegers. Does not modify the operands.
     * The Hamming distance is simply the amount of bits that differ between the two operands.
     * @param a the first operand
     * @param b the second operand
     * @return the Hamming distance between a and b
     */
    public static int hamming(SciInteger a, SciInteger b) {
        return hamming(a.ptr, b.ptr);
    }

    /**
     * Yield the smaller of two SciIntegers.
     * @param a the first operand
     * @param b the second operand
     * @return the smaller of a and b
     */
    public static SciInteger min(SciInteger a, SciInteger b) {
        if(a.lt(b)) {
            return a;
        } else {
            return b;
        }
    }

    /**
     * Yield the larger of two SciIntegers.
     * @param a the first operand
     * @param b the second operand
     * @return the larger of a and b
     */
    public static SciInteger max(SciInteger a, SciInteger b) {
        if(a.gt(b)) {
            return a;
        } else {
            return b;
        }
    }

    /**
     * Compute the integer square root of a SciInteger to produce a new SciInteger instance.
     * @param a the operand
     * @return a new SciInteger instance, the integer square root of a
     * @throws ArithmeticException if a is negative
     */
    public static SciInteger sqrt(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        sqrt(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the square of a SciInteger to produce a new SciInteger instance.
     * @param a the operand
     * @return a new SciInteger instance, the square of a
     */
    public static SciInteger square(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        square(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the value of the Legendre symbol of a and p.
     * @param a the first operand
     * @param p the second operand
     * @return the value of the Legendre symbol of a and p
     */
    public static int legendre(SciInteger a, SciInteger p) {
        return legendre(a.ptr, p.ptr);
    }

    /**
     * Compute the value of the Jacobi symbol of a and p.
     * @param a the first operand
     * @param p the second operand
     * @return the value of the Jacobi symbol of a and p
     */
    public static int jacobi(SciInteger a, SciInteger p) {
        return jacobi(a.ptr, p.ptr);
    }

    /**
     * Try to turn the SciInteger into a Java int.
     * @return the value of the SciInteger as a Java int
     * @throws ArithmeticException if the value of the SciInteger is too large to fit in a Java int
     */
    public int intValue() {
        return toInteger(ptr);
    }

    /**
     * Turn a SciInteger into a string. The string is formatted in base 10.
     * @return the value of the SciInteger as a string
     */
    @Override
    public String toString() {
        return toString(ptr);
    }

    /**
     * Turn a SciInteger into a string. The string is formatted in the given base.
     * @param radix the base to format the string in
     * @return the value of the SciInteger as a string
     * @throws IllegalArgumentException if radix is not between 2 and 36
     */
    public String toString(int radix) {
        return toStringRadix(ptr, radix);
    }

    /**
     * Return the value of applying the three-way comparison operator between two SciIntegers.
     * @param o the other SciInteger.
     * @return -1 if this is less than o, 0 if this is equal to o, 1 if this is greater than o
     */
    @Override
    public int compareTo(SciInteger o) {
        return compare(ptr, o.ptr);
    }

    /**
     * Copy a SciInteger to produce a new SciInteger instance.
     * @return a new SciInteger instance, a copy of `this`
     */
    @Override
    public SciInteger clone() {
        SciInteger result = SciInteger.fromInteger(0);
        copy(result.ptr, ptr);
        return result;
    }

    /**
     * Factor a SciInteger into its prime factors using the Pollard rho algorithm.
     * If a is negative, then the prime factors of -a are returned with an additional -1 factor in the result.
     * The result is a list of pairs of SciIntegers (represented as a {@link HashMap}), where the first element of the
     * pair is the prime factor and the second element is the exponent.
     * @param a the operand
     * @return the factorisation of a
     */
    public static HashMap<SciInteger, SciInteger> factor(SciInteger a) {
        HashMap<SciInteger, SciInteger> result = new HashMap<>();
        factor(result, a.ptr);
        return result;
    }

    /**
     * Compute the hash code of this object.
     * The value returned depends on the population count of this object. The primary design decision
     * that lead to this choice is to allow SciIntegers as keys for a HashMap, where we can easily bucket
     * them by the population count yielding a somewhat uniform distribution. This is not a great hashing solution,
     * but it's pretty darn fast.
     * @return the hash code of this object
     */
    @Override
    public int hashCode() {
        return Objects.hash(bitCount(this));
    }

    /**
     * Check for equality with another object. Takes care of funny cases like comparing a SciInteger to an object
     * of different type and two SciIntegers being physically equal before using the eq method.
     * @param obj the other object
     * @return true if the two objects are equal, false otherwise
     */
    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null) {
            return false;
        }
        if (getClass() != obj.getClass()) {
            return false;
        }
        final SciInteger other = (SciInteger) obj;
        if (this.ptr == other.ptr) {
            return true;
        }
        return this.eq(other);
    }
}
