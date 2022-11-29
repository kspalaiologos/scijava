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

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.MathContext;

public class TestSciFloat {
    public static final MathContext mc1 = new MathContext(100, MathContext.RoundingMode.NEAREST);
    public static final MathContext mc2 = new MathContext(200, MathContext.RoundingMode.NEAREST);

    @Test
    public void testAdd() {
        SciFloat a = SciFloat.valueOf(mc1, "182.5172735");
        SciFloat b = SciFloat.valueOf(mc1, "19.2958127");
        Assertions.assertEquals(SciFloat.add(mc1, a, b), SciFloat.valueOf(mc1, "201.8130862"));
    }

    @Test
    public void testSub() {
        SciFloat a = SciFloat.valueOf(mc1, "182.5172735");
        SciFloat b = SciFloat.valueOf(mc1, "19.2958127");
        Assertions.assertEquals(SciFloat.sub(mc1, a, b), SciFloat.valueOf(mc1, "163.2214608"));
    }

    @Test
    public void testMul() {
        SciFloat a = SciFloat.valueOf(mc1, "182.5172735");
        SciFloat b = SciFloat.valueOf(mc1, "19.2958127");
        Assertions.assertEquals(SciFloat.mul(mc1, a, b), SciFloat.valueOf(mc1, "3521.81912397067345"));
    }

    @Test
    public void testDiv() {
        SciFloat a = SciFloat.valueOf(mc1, "182.5172735");
        SciFloat b = SciFloat.valueOf(mc1, "19.2958127");
        Assertions.assertEquals(SciFloat.div(mc1, a, b), SciFloat.valueOf(mc1, "9.4589057396893161177917113592181"));
    }
}
