import static org.junit.Assert.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import org.junit.Test;

public class MainTest {

    @Test
    public void testE() {
        assertEquals(5, E.calc(new int[] { 2, 3, 1 }));
        assertEquals(30, E.calc(new int[] { 1, 2, 3, 4, 5 }));
        assertEquals(136, E.calc(new int[] { 8, 2, 7, 3, 4, 5, 6, 1 }));
    }

    @Test
    public void testC() {
        assertEquals(9, C.calc(new int[] { 9999, 2, 5 }));
        assertEquals(6, C.calc(new int[] { 9999, 3 }));
        assertEquals(53, C.calc(new int[] { 9999, 0, 153, 10, 10, 23 }));
    }

    @Test
    public void testB() {
        assertEquals(14, B.calc(new int[] { 3, 1, 2 }, new int[] { 2, 5, 4 }, new int[] { 3, 6 }));
        assertEquals(74, B.calc(new int[] { 2, 3, 4, 1 }, new int[] { 13, 5, 8, 24 }, new int[] { 45, 9, 15 }));
        assertEquals(150, B.calc(new int[] { 1, 2 }, new int[] { 50, 50 }, new int[] { 50 }));
    }

    @Test
    public void testA() {
        assertEquals(8, A.calc(2));
        assertEquals(1, A.calc(1));
    }
}
