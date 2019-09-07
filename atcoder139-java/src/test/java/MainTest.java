import static org.junit.Assert.*;

import org.junit.Test;

public class MainTest {

    @Test
    public void test() {
        assertEquals(2, P1.calc("CSS", "CSR"));
        assertEquals(3, P1.calc("SSR", "SSR"));

    }

    @Test
    public void test2() {
        assertEquals(3, P2.calc(4, 10));
        assertEquals(2, P2.calc(8, 9));
        assertEquals(1, P2.calc(8, 8));

    }

    @Test
    public void test3() {
        assertEquals(2, P3.calc(new int[] { 10, 4, 8, 7, 3 }));
        assertEquals(3, P3.calc(new int[] { 4, 4, 5, 6, 6, 5, 5 }));
        assertEquals(0, P3.calc(new int[] { 1, 2, 3, 4 }));
    }

    @Test
    public void test4() {
        assertEquals(1, P4.calc(2));
        assertEquals(78, P4.calc(13));
        assertEquals(0, P4.calc(1));
    }
}
