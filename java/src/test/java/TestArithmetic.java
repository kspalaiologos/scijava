import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import palaiologos.scijava.SciInteger;

import java.util.HashMap;

public class TestArithmetic {
    @Test
    public void testAdd() {
        Assertions.assertTrue(SciInteger.add(SciInteger.TEN, SciInteger.FIVE).eq(SciInteger.valueOf(15)));
    }

    @Test
    public void testSubtract() {
        Assertions.assertTrue(SciInteger.subtract(SciInteger.TEN, SciInteger.FIVE).eq(SciInteger.valueOf(5)));
    }

    @Test
    public void testMultiply() {
        Assertions.assertTrue(SciInteger.multiply(SciInteger.TEN, SciInteger.FIVE).eq(SciInteger.valueOf(50)));
    }

    @Test
    public void testDivide() {
        Assertions.assertTrue(SciInteger.divide(SciInteger.TEN, SciInteger.FIVE).eq(SciInteger.valueOf(2)));
    }

    @Test
    public void testAbs() {
        Assertions.assertTrue(SciInteger.abs(SciInteger.TEN).eq(SciInteger.TEN));
        Assertions.assertTrue(SciInteger.abs(SciInteger.valueOf(-10)).eq(SciInteger.TEN));
    }

    @Test
    public void testDivByZero() {
        Assertions.assertThrows(ArithmeticException.class, () -> SciInteger.divide(SciInteger.TEN, SciInteger.ZERO));
    }

    @Test
    public void testFromString() {
        Assertions.assertTrue(SciInteger.valueOf("10").eq(SciInteger.TEN));
    }

    @Test
    public void testfromStringRadix() {
        // test invalid radix (-1, 0, 100)
        Assertions.assertThrows(IllegalArgumentException.class, () -> SciInteger.valueOf("10", -1));
        Assertions.assertThrows(IllegalArgumentException.class, () -> SciInteger.valueOf("10", 0));
        Assertions.assertThrows(IllegalArgumentException.class, () -> SciInteger.valueOf("10", 100));
        // test base16
        Assertions.assertTrue(SciInteger.valueOf("A", 16).eq(SciInteger.TEN));
        // test base2
        Assertions.assertTrue(SciInteger.valueOf("1010", 2).eq(SciInteger.TEN));
    }

    @Test
    public void testModulus() {
        // try dividing by zero
        Assertions.assertThrows(ArithmeticException.class, () -> SciInteger.mod(SciInteger.TEN, SciInteger.ZERO));
        // try something tame
        Assertions.assertTrue(SciInteger.mod(SciInteger.TEN, SciInteger.FIVE).eq(SciInteger.ZERO));
        // try something more interesting
        Assertions.assertTrue(SciInteger.mod(SciInteger.TEN, SciInteger.valueOf(3)).eq(SciInteger.ONE));
    }

    @Test
    public void testPower() {
        // try a negative power
        Assertions.assertThrows(ArithmeticException.class, () -> SciInteger.pow(SciInteger.TEN, -1));
        // try a zero power
        Assertions.assertTrue(SciInteger.pow(SciInteger.TEN, 0).eq(SciInteger.ONE));
        // try a positive power
        Assertions.assertTrue(SciInteger.pow(SciInteger.TEN, 1).eq(SciInteger.TEN));
        // try a larger power
        Assertions.assertTrue(SciInteger.pow(SciInteger.TEN, 2).eq(SciInteger.valueOf(100)));
    }

    @Test
    public void testNegate() {
        Assertions.assertTrue(SciInteger.negate(SciInteger.TEN).eq(SciInteger.valueOf(-10)));
        // try double negation:
        Assertions.assertTrue(SciInteger.negate(SciInteger.negate(SciInteger.TEN)).eq(SciInteger.TEN));
    }

    @Test
    public void testGcd() {
        // try gcd of zero
        Assertions.assertTrue(SciInteger.gcd(SciInteger.ZERO, SciInteger.TEN).eq(SciInteger.TEN));
        // try gcd of two numbers
        Assertions.assertTrue(SciInteger.gcd(SciInteger.TEN, SciInteger.valueOf(15)).eq(SciInteger.FIVE));
        // try gcd of two numbers with a common factor
        Assertions.assertTrue(SciInteger.gcd(SciInteger.TEN, SciInteger.valueOf(20)).eq(SciInteger.TEN));
        // try negative gcd
        Assertions.assertTrue(SciInteger.gcd(SciInteger.TEN, SciInteger.valueOf(-15)).eq(SciInteger.FIVE));
    }

    @Test
    public void testLcm() {
        // apply the same methods as in testGcd.
        Assertions.assertTrue(SciInteger.lcm(SciInteger.ZERO, SciInteger.TEN).eq(SciInteger.ZERO));
        Assertions.assertTrue(SciInteger.lcm(SciInteger.TEN, SciInteger.valueOf(15)).eq(SciInteger.valueOf(30)));
        Assertions.assertTrue(SciInteger.lcm(SciInteger.TEN, SciInteger.valueOf(20)).eq(SciInteger.valueOf(20)));
        Assertions.assertTrue(SciInteger.lcm(SciInteger.TEN, SciInteger.valueOf(-15)).eq(SciInteger.valueOf(30)));
    }

    @Test
    public void testFactorials() {
        // try a negative factorial
        Assertions.assertThrows(ArithmeticException.class, () -> SciInteger.factorial(-1));
        // try a zero factorial
        Assertions.assertTrue(SciInteger.factorial(0).eq(SciInteger.ONE));
        // try a positive factorial
        Assertions.assertTrue(SciInteger.factorial(10).eq(SciInteger.valueOf(3628800)));
        // try something bigger.
        Assertions.assertTrue(SciInteger.factorial(20).eq(SciInteger.valueOf("2432902008176640000")));
        // try factorial of 50
        Assertions.assertTrue(SciInteger.factorial(50).eq(SciInteger.valueOf("30414093201713378043612608166064768844377641568960512000000000000")));
    }

    @Test
    public void testSignum() {
        // try a positive number
        Assertions.assertTrue(SciInteger.signum(SciInteger.TEN).eq(SciInteger.ONE));
        // try a negative number
        Assertions.assertTrue(SciInteger.signum(SciInteger.valueOf(-10)).eq(SciInteger.valueOf(-1)));
        // try zero
        Assertions.assertTrue(SciInteger.signum(SciInteger.ZERO).eq(SciInteger.ZERO));
    }

    @Test
    public void testToString() {
        // try a positive number
        Assertions.assertEquals("10", SciInteger.TEN.toString());
        // try a negative number
        Assertions.assertEquals("-10", SciInteger.valueOf(-10).toString());
        // try zero
        Assertions.assertEquals("0", SciInteger.ZERO.toString());
        // try some other radix
        Assertions.assertEquals("1010", SciInteger.TEN.toString(2));
        // try bad radix
        Assertions.assertThrows(IllegalArgumentException.class, () -> SciInteger.TEN.toString(-1));
    }

    @Test
    public void testFactor() {
        // 2^4 * 3^2 * 5 = 720
        var factors = SciInteger.factor(SciInteger.valueOf(720));
        Assertions.assertEquals(3, factors.size());
        Assertions.assertTrue(factors.get(SciInteger.valueOf(2)).eq(SciInteger.valueOf(4)));
        Assertions.assertTrue(factors.get(SciInteger.valueOf(3)).eq(SciInteger.valueOf(2)));
        Assertions.assertTrue(factors.get(SciInteger.valueOf(5)).eq(SciInteger.ONE));
        // -1 * 2^4 * 3^2 * 5 = -720
        factors = SciInteger.factor(SciInteger.valueOf(-720));
        Assertions.assertEquals(4, factors.size());
        Assertions.assertTrue(factors.get(SciInteger.valueOf(-1)).eq(SciInteger.ONE));
        Assertions.assertTrue(factors.get(SciInteger.valueOf(2)).eq(SciInteger.valueOf(4)));
        Assertions.assertTrue(factors.get(SciInteger.valueOf(3)).eq(SciInteger.valueOf(2)));
        Assertions.assertTrue(factors.get(SciInteger.valueOf(5)).eq(SciInteger.ONE));
        // factor 0
        factors = SciInteger.factor(SciInteger.ZERO);
        Assertions.assertEquals(0, factors.size());
    }

    @Test
    public void testClone() {
        // try cloning a positive number
        Assertions.assertTrue(SciInteger.TEN.clone().eq(SciInteger.TEN));
        // try cloning a negative number
        Assertions.assertTrue(SciInteger.valueOf(-10).clone().eq(SciInteger.valueOf(-10)));
        // try cloning zero
        Assertions.assertTrue(SciInteger.ZERO.clone().eq(SciInteger.ZERO));
    }

    @Test
    public void testIntValue() {
        // try a positive number
        Assertions.assertEquals(10, SciInteger.TEN.intValue());
        // try a negative number
        Assertions.assertEquals(-10, SciInteger.valueOf(-10).intValue());
        // try zero
        Assertions.assertEquals(0, SciInteger.ZERO.intValue());
        // try to overflow
        Assertions.assertThrows(ArithmeticException.class, () -> SciInteger.valueOf("9e999").intValue());
    }

    @Test
    public void testSquareSqrt() {
        // try a positive number
        Assertions.assertTrue(SciInteger.sqrt(SciInteger.square(SciInteger.TEN)).eq(SciInteger.TEN));
        // try a negative number
        Assertions.assertTrue(SciInteger.sqrt(SciInteger.square(SciInteger.valueOf(-10))).eq(SciInteger.valueOf(10)));
        // try zero
        Assertions.assertTrue(SciInteger.sqrt(SciInteger.ZERO).eq(SciInteger.ZERO));
    }
}
