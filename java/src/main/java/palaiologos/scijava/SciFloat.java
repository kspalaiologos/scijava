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

public final class SciFloat {
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
    private static native boolean eq(long a, long b);
    private static native SciFloat fromString(int precision, int roundingMode, String s);
    private static native SciFloat fromInteger(int precision, int roundingMode, int n);

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

    public boolean eq(SciFloat other) {
        return eq(ptr, other.ptr);
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
}
