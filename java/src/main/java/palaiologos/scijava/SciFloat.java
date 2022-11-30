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

public final class SciFloat implements Comparable<SciFloat>, Cloneable {
    static {
        try {
            load(resourceName());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private final long ptr;

    private final Cleaner.Cleanable cleanable;

    private SciFloat(long ptr) {
        this.ptr = ptr;
        cleanable = CleanerSingleton.CLEANER.register(this, () -> {
            SciFloat.free(ptr);
        });
    }

    private static native void free(long ptr);
    private static native String toString(long i);
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
    private static native void cbrt(int precision, int roundingMode, long dest, long a);
    private static native void neg(int precision, int roundingMode, long dest, long a);
    private static native void abs(int precision, int roundingMode, long dest, long a);
    private static native void factorial(int precision, int roundingMode, long dest, int a);
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
    private static native boolean isFinite(long ptr);

    // Public API
    public static SciFloat valueOf(MathContext mc, int n) {
        return fromInteger(mc.precision(), mc.roundingMode().ordinal(), n);
    }

    public static SciFloat valueOf(MathContext mc, String s) {
        return fromString(mc.precision(), mc.roundingMode().ordinal(), s);
    }

    public static SciFloat add(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.add(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciFloat sub(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sub(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciFloat mul(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.mul(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciFloat div(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.div(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciFloat mod(MathContext mc, SciFloat a, SciFloat b) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.mod(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr, b.ptr);
        return result;
    }

    public static SciFloat sqrt(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sqrt(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat sin(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sin(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat cos(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cos(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat tan(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.tan(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat sec(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.sec(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat csc(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.csc(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat cot(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cot(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat cbrt(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.cbrt(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat neg(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.neg(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat abs(MathContext mc, SciFloat a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.abs(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a.ptr);
        return result;
    }

    public static SciFloat factorial(MathContext mc, int a) {
        SciFloat result = SciFloat.valueOf(mc, 0);
        SciFloat.factorial(mc.precision(), mc.roundingMode().ordinal(), result.ptr, a);
        return result;
    }

    public boolean isFinite() {
        return isFinite(ptr);
    }

    public boolean eq(SciFloat other) {
        return eq(ptr, other.ptr);
    }

    public boolean neq(SciFloat other) {
        return neq(ptr, other.ptr);
    }

    public boolean lt(SciFloat other) {
        return lt(ptr, other.ptr);
    }

    public boolean lte(SciFloat other) {
        return lte(ptr, other.ptr);
    }

    public boolean gt(SciFloat other) {
        return gt(ptr, other.ptr);
    }

    public boolean gte(SciFloat other) {
        return gte(ptr, other.ptr);
    }

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

    @Override
    public String toString() {
        return toString(ptr);
    }

    @Override
    public int compareTo(SciFloat o) {
        return compare(ptr, o.ptr);
    }

    @Override
    protected Object clone() {
        SciFloat result = SciFloat.valueOf(getMathContext(ptr), 0);
        copy(result.ptr, ptr);
        return result;
    }
}
