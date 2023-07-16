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

    private static native void add(long dest, long a, long b);
    private static native void sub(long dest, long a, long b);

    /**
     * Return a new SciInteger with the value of the specified integer.
     * @param i the integer to convert
     * @return a new SciInteger instance
     */
    public static SciRational valueOf(int i) {
        return fromInteger(i);
    }

    /**
     * Add two SciIntegers to produce a new SciInteger instance.
     * Does not modify the operands.
     * @param a the first operand
     * @param b the second operand
     * @return a new SciInteger instance, the result of a + b
     */
    public static SciRational add(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
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
    public static SciRational subtract(SciRational a, SciRational b) {
        SciRational result = SciRational.fromInteger(0);
        sub(result.ptr, a.ptr, b.ptr);
        return result;
    }

    @Override
    public int hashCode() {
        throw new UnsupportedOperationException();
    }

    @Override
    public int compareTo(SciRational o) {
        throw new UnsupportedOperationException();
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
        return this.eq(other);
    }
}
