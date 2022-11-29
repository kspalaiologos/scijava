import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import palaiologos.scijava.SciInteger;

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
}
