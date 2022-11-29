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

public class SciInteger implements Comparable<SciInteger>, Cloneable {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciInteger(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, () -> {
            SciInteger.free(ptr);
        });
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
    private static native void testBit(long dest, long a, int b);
    private static native int bitCount(long a);
    private static native int bitLength(long a);
    private static native boolean isPrime(long a);
    private static native void nextPrime(long dest, long a);
    private static native void clamp(long dest, long a, long min, long max);
    private static native void divmod(long destdiv, long destmod, long a, long b);
    private static native void fibonacci(long dest, int a);
    private static native void lucas(long dest, int a);
    private static native int hamming(long a, long b);
    private static native void sqrt(long dest, long a);
    private static native void square(long dest, long a);
    private static native int jacobi(long a, long b);
    private static native int legendre(long a, long b);
    private static native SciInteger fromInteger(int i);
    private static native SciInteger fromString(String s);
    private static native SciInteger fromStringRadix(String s, int radix);
    private static native int toInteger(long i);
    private static native void copy(long dest, long src);
    private static native void factor(Map<SciInteger, SciInteger> destFactors, long a);

    // Public API.
    public static final SciInteger ZERO = fromInteger(0);
    public static final SciInteger ONE = fromInteger(1);
    public static final SciInteger TWO = fromInteger(2);
    public static final SciInteger FIVE = fromInteger(5);
    public static final SciInteger TEN = fromInteger(10);

    public static SciInteger valueOf(int i) {
        return fromInteger(i);
    }

    public static SciInteger valueOf(String s) {
        return fromString(s);
    }

    public static SciInteger valueOf(String s, int radix) {
        return fromStringRadix(s, radix);
    }

    public static SciInteger add(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        add(result.ptr, a.ptr, b.ptr);
        return result;
    }
    
    public static SciInteger subtract(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        sub(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger multiply(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        mul(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger divide(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        div(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger mod(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        rem(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger pow(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        pow(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger negate(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        negate(result.ptr, a.ptr);
        return result;
    }

    public static SciInteger abs(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        abs(result.ptr, a.ptr);
        return result;
    }

    public static SciInteger gcd(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        gcd(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger lcm(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        lcm(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger factorial(int a) {
        SciInteger result = SciInteger.fromInteger(0);
        factorial(result.ptr, a);
        return result;
    }

    public static SciInteger signum(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        signum(result.ptr, a.ptr);
        return result;
    }

    public boolean lt(SciInteger b) {
        return lt(ptr, b.ptr);
    }

    public boolean lte(SciInteger b) {
        return lte(ptr, b.ptr);
    }

    public boolean gt(SciInteger b) {
        return gt(ptr, b.ptr);
    }

    public boolean gte(SciInteger b) {
        return gte(ptr, b.ptr);
    }

    public boolean eq(SciInteger b) {
        return eq(ptr, b.ptr);
    }

    public boolean neq(SciInteger b) {
        return neq(ptr, b.ptr);
    }

    public static SciInteger and(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        and(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger or(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        or(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger xor(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        xor(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger not(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        not(result.ptr, a.ptr);
        return result;
    }

    public static SciInteger shl(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        shl(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger shr(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        shr(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger setBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        setBit(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger clearBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        clearBit(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger testBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        testBit(result.ptr, a.ptr, b);
        return result;
    }

    public static SciInteger flipBit(SciInteger a, int b) {
        SciInteger result = SciInteger.fromInteger(0);
        flipBit(result.ptr, a.ptr, b);
        return result;
    }

    public static int bitLength(SciInteger a) {
        return bitLength(a.ptr);
    }

    public static int bitCount(SciInteger a) {
        return bitCount(a.ptr);
    }

    public static boolean isPrime(SciInteger a) {
        return isPrime(a.ptr);
    }

    public static SciInteger nextPrime(SciInteger a) {
        SciInteger result = SciInteger.valueOf(0);
        nextPrime(result.ptr, a.ptr);
        return result;
    }

    public static SciInteger[] divMod(SciInteger a, SciInteger b) {
        SciInteger[] tab = new SciInteger[2];
        tab[0] = SciInteger.fromInteger(0);
        tab[1] = SciInteger.fromInteger(0);
        divmod(tab[0].ptr, tab[1].ptr, a.ptr, b.ptr);
        return tab;
    }

    public static SciInteger fibonacci(int n) {
        SciInteger result = SciInteger.fromInteger(0);
        fibonacci(result.ptr, n);
        return result;
    }

    public static SciInteger clamp(SciInteger a, SciInteger min, SciInteger max) {
        SciInteger result = SciInteger.fromInteger(0);
        clamp(result.ptr, a.ptr, min.ptr, max.ptr);
        return result;
    }

    public static SciInteger lucas(int n) {
        SciInteger result = SciInteger.fromInteger(0);
        lucas(result.ptr, n);
        return result;
    }

    public static SciInteger hamming(int n) {
        SciInteger result = SciInteger.fromInteger(0);
        hamming(result.ptr, n);
        return result;
    }

    public static SciInteger min(SciInteger a, SciInteger b) {
        if(a.lt(b)) {
            return a;
        } else {
            return b;
        }
    }

    public static SciInteger max(SciInteger a, SciInteger b) {
        if(a.gt(b)) {
            return a;
        } else {
            return b;
        }
    }

    public static SciInteger sqrt(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        sqrt(result.ptr, a.ptr);
        return result;
    }

    public static SciInteger square(SciInteger a) {
        SciInteger result = SciInteger.fromInteger(0);
        square(result.ptr, a.ptr);
        return result;
    }

    public static int legendre(SciInteger a, SciInteger p) {
        return legendre(a.ptr, p.ptr);
    }

    public static int jacobi(SciInteger a, SciInteger p) {
        return jacobi(a.ptr, p.ptr);
    }

    public int intValue() {
        return toInteger(ptr);
    }

    @Override
    public String toString() {
        return toString(ptr);
    }

    public String toString(int radix) {
        return toStringRadix(ptr, radix);
    }

    @Override
    public int compareTo(SciInteger o) {
        return compare(ptr, o.ptr);
    }

    @Override
    public SciInteger clone() {
        SciInteger result = SciInteger.fromInteger(0);
        copy(result.ptr, ptr);
        return result;
    }

    public HashMap<SciInteger, SciInteger> factor(SciInteger a) {
        HashMap<SciInteger, SciInteger> result = new HashMap<>();
        factor(result, a.ptr);
        return result;
    }

    @Override
    public int hashCode() {
        return Objects.hash(ptr);
    }
}
