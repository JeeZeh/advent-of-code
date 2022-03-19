package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class SolutionTest {
    @Test
    public void shouldPassPartOneSampleCases() {
        assertEquals("0124515891", new Solution("5").partOne());
        assertEquals("5158916779", new Solution("9").partOne());
        assertEquals("9251071085", new Solution("18").partOne());
        assertEquals("5941429882", new Solution("2018").partOne());
    }
    
    @Test
    public void shouldPassPartTwoSampleCases() {
        assertEquals("5", new Solution("01245").partTwo());
        assertEquals("9", new Solution("51589").partTwo());
        assertEquals("18", new Solution("92510").partTwo());
        assertEquals("2018", new Solution("59414").partTwo());
    }
}
