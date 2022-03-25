package aoc;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class PointTest {
    @Test
    public void testAdd() {
        assertEquals((new Point(0, 0)).add(new Point(1, 1)), new Point(1, 1));
        assertEquals((new Point(0, 0)).add(new Point(-1, -1)), new Point(-1, -1));
    }

    @Test
    public void testCompareToLessThan() {
        Point a = new Point(1, 1);
        Point b = new Point(1, 2);
        Point c = new Point(2, 1);
        Point d = new Point(2, 2);
        assertEquals(a.compareTo(b), -1);
        assertEquals(a.compareTo(c), -1);
        assertEquals(a.compareTo(d), -1);
    }

    @Test
    public void testCompareToEqual() {
        Point a = new Point(1, 1);
        Point b = new Point(1, 1);
        Point c = new Point(0, 0);
        Point d = new Point(0, 0);
        assertEquals(a.compareTo(b), 0);
        assertEquals(c.compareTo(d), 0);
    }

    @Test
    public void testCompareToGreaterThan() {
        Point a = new Point(1, 1);
        Point b = new Point(1, 2);
        Point c = new Point(2, 1);
        Point d = new Point(0, 2);
        assertEquals(b.compareTo(a), 1);
        assertEquals(c.compareTo(a), 1);
        assertEquals(d.compareTo(a), 1);
    }
}
