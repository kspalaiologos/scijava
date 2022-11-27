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

import java.lang.ref.Cleaner;

public class SciInteger {
    private final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciInteger(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, () -> {
            SciInteger.free(ptr);
        });
    }

    static {
        System.loadLibrary("scijava");
    }

    private static native void free(long ptr);
    
    private static native String toString(long i);
    private static native String toStringRadix(long i, int radix);

    private static native void add(long dest, long a, long b);
    private static native void subtract(long dest, long a, long b);
    private static native void multiply(long dest, long a, long b);
    private static native void divide(long dest, long a, long b);
    private static native void mod(long dest, long a, long b);
    private static native void pow(long dest, long a, int b);
    private static native void negate(long dest, long a);
    private static native void abs(long dest, long a);
    private static native void gcd(long dest, long a, long b);
    private static native void lcm(long dest, long a, long b);
    private static native void factorial(long dest, int a);

    // Public API.
    public static native SciInteger fromInteger(int i);
    public static native SciInteger fromString(String s);
    public static native SciInteger fromStringRadix(String s, int radix);

    public static SciInteger add(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        add(result.ptr, a.ptr, b.ptr);
        return result;
    }
    
    public static SciInteger subtract(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        subtract(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger multiply(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        multiply(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger divide(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        divide(result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciInteger mod(SciInteger a, SciInteger b) {
        SciInteger result = SciInteger.fromInteger(0);
        mod(result.ptr, a.ptr, b.ptr);
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

    @Override
    public String toString() {
        return toString(ptr);
    }

    public String toString(int radix) {
        return toStringRadix(ptr, radix);
    }
}
