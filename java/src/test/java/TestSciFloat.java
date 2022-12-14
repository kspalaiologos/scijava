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
import palaiologos.scijava.differentiation.DerivativeDirection;
import palaiologos.scijava.differentiation.RealDifferentiation;
import palaiologos.scijava.integrator.Integrator;
import palaiologos.scijava.integrator.RealFunction;
import palaiologos.scijava.SciFloat;
import palaiologos.scijava.MathContext;
import palaiologos.scijava.integrator.RealGaussLegendreIntegrator;
import palaiologos.scijava.integrator.RealTanhSinhIntegrator;
import palaiologos.scijava.sum.EulerMaclaurinSum;
import palaiologos.scijava.util.Pair;

public class TestSciFloat {
    public static final MathContext mc1 = new MathContext(100, MathContext.RoundingMode.NEAREST);
    public static final MathContext mc10 = new MathContext(1000, MathContext.RoundingMode.NEAREST);

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

    @Test
    public void testDivByZero() {
        SciFloat a = SciFloat.valueOf(mc1, "182.5172735");
        SciFloat b = SciFloat.valueOf(mc1, "0");
        Assertions.assertFalse(SciFloat.div(mc1, a, b).isFinite());
    }

    @Test
    public void testLambert() {
        SciFloat a = SciFloat.valueOf(mc10, "1.23");
        SciFloat b = SciFloat.valueOf(mc10, "0.645203569593202377590356052553348538301733002626664912115203797900052222847426796504119143142348699969178659329439934993915874680405142664275481137608381865116289455326427098548598099125440343380476232961304953162567790577710762271767537658970067993842396166397437914122577514488042093297500281715494069");
        Assertions.assertEquals(SciFloat.lambertw(mc10, a, 0), b);
    }

    @Test
    public void testBernoulli() {
        Assertions.assertEquals(SciFloat.bernoulli(mc1, 6), SciFloat.valueOf(mc1, "2.3809523809523809523809523809519e-2"));
        Assertions.assertEquals(SciFloat.bernoulli(mc1, 20), SciFloat.valueOf(mc1, "-529.12424242424242424242424242406"));
        Assertions.assertEquals(SciFloat.bernoulli(mc1, 30), SciFloat.valueOf(mc1, "601580873.90064236838430386817462"));
        Assertions.assertEquals(SciFloat.bernoulli(mc1, 50), SciFloat.valueOf(mc1, "7500866746076964366855720.0757599"));
        Assertions.assertEquals(SciFloat.bernoulli(mc10, 22), SciFloat.valueOf(mc10, "6192.12318840579710144927536231884057971014492753623188405797101449275362318840579710144927536231884057971014492753623188405797101449275362318840579710144927536231884057971014492753623188405797101449275362318840579710144927536231884057971014492753623188405797101449275362318840579710144927536231884057956"));
    }

    @Test
    public void testQuadrature() {
        Pair<SciFloat, SciFloat> result;

        // TanhSinh
        result = RealTanhSinhIntegrator.quad(mc1, (mc, x) -> SciFloat.exp(mc, SciFloat.neg(mc, SciFloat.mul(mc, x, x))), new SciFloat[] { SciFloat.MINUS_ONE, SciFloat.ONE });

        Assertions.assertEquals(result.left, SciFloat.valueOf(mc1, "1.4936482656248540507989348722634"));

        result = RealTanhSinhIntegrator.quad(mc1, (mc, x) -> SciFloat.exp(mc, SciFloat.neg(mc, SciFloat.mul(mc, x, x))), new SciFloat[] { SciFloat.NINF, SciFloat.INF });

        Assertions.assertEquals(SciFloat.mul(mc1, result.left, result.left), SciFloat.pi(mc1));

        // Gauss-Legendre
        result = RealGaussLegendreIntegrator.quad(mc1, (mc, x) -> {
            return SciFloat.exp(mc, SciFloat.neg(mc, SciFloat.mul(mc, x, x)));
        }, new SciFloat[] { SciFloat.NINF, SciFloat.INF });

        Assertions.assertEquals(SciFloat.mul(mc1, result.left, result.left), SciFloat.pi(mc1));

        // Integrate gamma(x)/x^x from 1 to infinity.
        result = RealGaussLegendreIntegrator.quad(mc1, (mc, x) -> {
            return SciFloat.div(mc, SciFloat.gamma(mc, x), SciFloat.pow(mc, x, x));
        }, new SciFloat[] { SciFloat.ONE, SciFloat.INF });

        Assertions.assertEquals(result.left, SciFloat.valueOf(mc1, "0.73657685272323505319808654132535"));
    }

    @Test
    public void testLerchPhi() {
        SciFloat a = SciFloat.valueOf(mc1, "0");
        SciFloat b = SciFloat.valueOf(mc1, "1");
        SciFloat c = SciFloat.valueOf(mc1, "2");
        Assertions.assertEquals(SciFloat.lerchPhi(mc1, a, b, c), SciFloat.valueOf(mc1, "0.5"));
    }

    @Test
    public void testDifferentiation() {
        SciFloat result = RealDifferentiation.differentiate(
                mc1, (mc, x) ->
                        SciFloat.add(mc1, SciFloat.mul(mc1, x, x), x),
                SciFloat.ONE, 1, DerivativeDirection.CENTRAL, 0, false, false);
        Assertions.assertEquals(result, SciFloat.valueOf(mc1, "2"));
    }

    @Test
    public void testEulerMaclaurin() {
        // sum of gamma(x)/x^x from 1 to infinity.
        Pair<SciFloat, SciFloat> s = EulerMaclaurinSum.sum(mc1, (mc, x) -> SciFloat.div(mc, SciFloat.gamma(mc, x), SciFloat.pow(mc, x, x)), SciFloat.ONE, SciFloat.INF, null, null, null, Integrator.GAUSS_LEGENDRE);
        Assertions.assertEquals(s.left, SciFloat.valueOf(mc1, "1.3563736992032889678785295836750701"));
    }

    @Test
    public void testHurwitz() {
        SciFloat h = SciFloat.hurwitzZeta(mc1, SciFloat.valueOf(mc1, "1.5"), SciFloat.valueOf(mc1, "2.5"));
        Assertions.assertEquals(h, SciFloat.valueOf(mc1, "1.4037839632432208564079768276140"));
    }
}
