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

/**
 * A record encapsulating the context settings which describe rules for numerical
 * operations, such as those implemented by {@link SciFloat}.
 *
 * The settings are:
 * <ul>
 *     <li>precision: the number of <b>binary</b> digits not smaller than one to be used for an operation's result</li>
 *     <li>rounding mode: the algorithm to be used for rounding</li>
 * </ul>
 *
 * @param precision The precision in binary digits represented by this object.
 * @param roundingMode The rounding mode represented by this object.
 * @author Kamila Szewczyk
 * @see SciFloat
 */
public record MathContext(int precision, palaiologos.scijava.MathContext.RoundingMode roundingMode) {
    public MathContext {
        if (precision < 1) {
            throw new IllegalArgumentException("precision < 1");
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

    /**
     * @return the string representation of this {@code MathContext} object.
     */
    @Override
    public String toString() {
        return "MathContext{" +
                "precision=" + precision +
                ", roundingMode=" + roundingMode +
                '}';
    }

    // Some default contexts:
    public static final MathContext MC53 = new MathContext(53, RoundingMode.NEAREST);
    public static final MathContext MC24 = new MathContext(24, RoundingMode.NEAREST);
}
