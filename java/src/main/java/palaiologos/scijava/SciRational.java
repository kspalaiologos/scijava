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
 * Arbitrary precision rationals provided by SciJava.
 *
 * @author Kamila Szewczyk
 */
public final class SciRational implements Comparable<SciRational>, Cloneable {
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
            SciRational.free(pointer);
        }
    }

    final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciRational(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, new CleanerRunnable(ptr));
    }

    private static native void free(long ptr);
    
    private static native String toString(long i);
    private static native String toStringRadix(long i, int radix);
    private static native SciRational fromInteger(int i);
    private static native SciRational fromSciFloat(long f);
    private static native SciRational fromSciInteger(long f);
    private static native SciInteger num(long a);
    private static native SciInteger den(long a);
    private static native long hash(long a);
    private static native SciRational fromString(String s);
    private static native SciRational fromStringRadix(String s, int radix);
    private static native boolean lt(long a, long b);
    private static native boolean lte(long a, long b);
    private static native boolean gt(long a, long b);
    private static native boolean gte(long a, long b);
    private static native boolean eq(long a, long b);
    private static native boolean neq(long a, long b);
    private static native void copy(long dest, long src);
    private static native int compare(long a, long b);
    private static native void add(long dest, long a, long b);
    private static native void sub(long dest, long a, long b);
    private static native void mul(long dest, long a, long b);
    private static native void div(long dest, long a, long b);
    private static native void pow(long dest, long a, int b);
    private static native void recip(long dest, long a);
    private static native void ceil(long dest, long a);
    private static native void floor(long dest, long a);
    private static native void abs(long dest, long a);
    private static native void round(long dest, long a);
    private static native void fract_ceil(long dest, long dest2, long b);
    private static native void fract_floor(long dest, long dest2, long b);
    private static native void fract_round(long dest, long dest2, long b);
    private static native void fract_trunc(long dest, long dest2, long b);
    private static native void rem_ceil(long dest, long dest2, long b);
    private static native void rem_floor(long dest, long dest2, long b);
    private static native void rem_round(long dest, long dest2, long b);
    private static native void rem_trunc(long dest, long dest2, long b);
    private static native int signum(long a);
    private static native void square(long dest, long a);
    private static native void neg(long dest, long a);

    /**
     * Return a new SciRational with the value of the specified integer.
     * @param i
     * @return a new SciRational instance
     */
    public static SciRational valueOf(SciInteger i) {
        return fromSciInteger(i.ptr);
    }

    /**
     * Return a new SciRational with the value of the specified float.
     * @param i
     * @return a new SciRational instance
     */
    public static SciRational valueOf(SciFloat i) {
        return fromSciFloat(i.ptr);
    }

    /**
     * Return a new SciRational with the value represented by
     * the given string.
     * @param s
     * @return a new SciRational instance
     */
    public static SciRational valueOf(String s) {
        return fromString(s);
    }

    /**
     * Return a new SciRational with the value represented by
     * the given string in a specified radix.
     * @param s
     * @param radix
     * @return a new SciRational instance
     */
    public static SciRational valueOf(String s, int radix) {
        return fromStringRadix(s, radix);
    }

    /**
     * Return a new SciInteger with the value of the specified integer.
     * @param i the integer to convert
     * @return a new SciInteger instance
     */
    public static SciRational valueOf(int i) {
        return fromInteger(i);
    }

    /**
     * Obtain the numerator of this SciRational value.
     * @return the numerator
     */
    public SciInteger numerator() {
        return num(ptr);
    }

    /**
     * Obtain the denominator of this SciRational value.
     * @return the denominator
     */
    public SciInteger denominator() {
        return den(ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * @param b the other SciRational
     * @return true if this SciRational is less than b, false otherwise
     */
    public boolean lt(SciRational b) {
        return lt(ptr, b.ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * @param b the other SciRational
     * @return true if this SciRational is less than or equal to b, false otherwise
     */
    public boolean lte(SciRational b) {
        return lte(ptr, b.ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * @param b the other SciRational
     * @return true if this SciRational is greater than b, false otherwise
     */
    public boolean gt(SciRational b) {
        return gt(ptr, b.ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * @param b the other SciInteger
     * @return true if this SciInteger is greater than or equal to b, false otherwise
     */
    public boolean gte(SciRational b) {
        return gte(ptr, b.ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * Notice that this method always involves the underlying equality checks. When dealing
     * with objects of unknown origin, consider using the {@link #equals(Object)} method first
     * to avoid unnecessary equality checks in case the objects are physically equal or the
     * types differ.
     * @param b the other SciInteger
     * @return true if this SciInteger is equal to b, false otherwise
     */
    public boolean eq(SciRational b) {
        return eq(ptr, b.ptr);
    }

    /**
     * Compare this SciRational with another SciRational to determine their relative order.
     * Notice that this method always involves the underlying equality checks. When dealing
     * with objects of unknown origin, consider using the {@link #equals(Object)} method first
     * to avoid unnecessary equality checks in case the objects are physically equal or the
     * types differ.
     * @param b the other SciRational
     * @return true if this SciRational is not equal to b, false otherwise
     */
    public boolean neq(SciRational b) {
        return neq(ptr, b.ptr);
    }

    /**
     * Add two SciRationals to produce a new SciRational instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciRational instance, the result of a + b
     */
    public static SciRational add(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
        add(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Subtract two SciRationals to produce a new SciRational instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciRational instance, the result of a - b
     */
    public static SciRational subtract(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
        sub(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Multiply two SciRationals to produce a new SciRational instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciRational instance, the result of a * b
     */
    public static SciRational multiply(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
        mul(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Divide two SciRationals to produce a new SciRational instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciRational instance, the result of a / b
     */
    public static SciRational divide(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
        div(result.ptr, a.ptr, b.ptr);
        return result;
    }

    /**
     * Raise a SciRational to an integer power to produce a
     * new SciRational instance.
     * @param a
     * @param exp
     * @return a new SciRational instance, the result of a ^ exp
     */
    public static SciRational pow(SciRational a, int exp) {
        SciRational result = SciRational.fromInteger(0);
        pow(result.ptr, a.ptr, exp);
        return result;
    }

    /**
     * Compute the reciprocal of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of 1 / a
     */
    public static SciRational recip(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        recip(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the floor function of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of floor(a)
     */
    public static SciRational floor(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        floor(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the ceiling function of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of ceil(a)
     */
    public static SciRational ceil(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        ceil(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the absolute value of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of abs(a)
     */
    public static SciRational abs(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        abs(result.ptr, a.ptr);
        return result;
    }

    /**
     * Round a SciRational to the nearest integer to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of round(a)
     */
    public static SciRational round(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        round(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the sign of a SciRational.
     * @param a
     * @return -1 if a < 0, 0 if a == 0, 1 if a > 0
     */
    public static int signum(SciRational a) {
        return signum(a.ptr);
    }

    /**
     * Efficiently compute the square of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of a * a
     */
    public static SciRational square(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        square(result.ptr, a.ptr);
        return result;
    }

    /**
     * Negate the value of a SciRational to produce a
     * new SciRational instance.
     * @param a
     * @return a new SciRational instance, the result of -a
     */
    public static SciRational neg(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        neg(result.ptr, a.ptr);
        return result;
    }

    /**
     * Compute the fractional and ceil parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> fractCeil(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        fract_ceil(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the fractional and floor parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> fractFloor(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        fract_floor(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the fractional and round parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> fractRound(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        fract_round(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the fractional and truncated parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> fractTrunc(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        fract_trunc(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the remainder and ceil parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> remCeil(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        rem_ceil(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the remainder and floor parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> remFloor(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        rem_floor(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the remainder and round parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> remRound(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        rem_round(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Compute the remainder and truncated parts of a SciRational.
     * @param a
     * @return
     */
    public static Pair<SciRational, SciInteger> remTrunc(SciRational a) {
        SciRational result = SciRational.fromInteger(0);
        SciInteger intPart = SciInteger.fromInteger(0);
        rem_trunc(result.ptr, intPart.ptr, a.ptr);
        return new Pair<>(result, intPart);
    }

    /**
     * Make a copy of this SciRational.
     * @return
     */
    @Override
    public SciRational clone() {
        SciRational result = SciRational.fromInteger(0);
        copy(result.ptr, ptr);
        return result;
    }

    /**
     * Compute the hash code of this SciRational.
     * @return
     */
    @Override
    public int hashCode() {
        return (int) hash(ptr);
    }

    /**
     * Compare this SciRational with another object.
     * @param o the object to be compared.
     * @return
     */
    @Override
    public int compareTo(SciRational o) {
        return compare(ptr, o.ptr);
    }

    /**
     * Turn a SciRational into a string. The string is formatted in base 10.
     * @return the value of the SciRational as a string
     */
    @Override
    public String toString() {
        return toString(ptr);
    }

    /**
     * Turn a SciRational into a string. The string is formatted in the given base.
     * @param radix the base to format the string in
     * @return the value of the SciRational as a string
     * @throws IllegalArgumentException if radix is not between 2 and 36
     */
    public String toString(int radix) {
        return toStringRadix(ptr, radix);
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
        final SciRational other = (SciRational) obj;
        if (this.ptr == other.ptr) {
            return true;
        }
        return eq(this.ptr, other.ptr);
    }
}
