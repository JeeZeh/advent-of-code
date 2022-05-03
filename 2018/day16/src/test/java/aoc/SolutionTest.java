package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.util.List;

import org.junit.Test;

public class SolutionTest {
    @Test
    public void TestCase_matchesCorrectCases() {
        /**
         * Before: [3, 2, 1, 1]
         * 9 2 1 2
         * After: [3, 2, 2, 1]
         */
        TestCase test = new TestCase(9, new int[] { 3, 2, 1, 1 }, new Operands(2, 1, 2), new int[] { 3, 2, 2, 1 });
        List<String> found = test.run().matching;
        assertEquals(3, found.size());
        assertTrue(found.contains("mulr"));
        assertTrue(found.contains("addi"));
        assertTrue(found.contains("seti"));
    }
}
