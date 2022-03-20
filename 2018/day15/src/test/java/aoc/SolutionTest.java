package aoc;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class SolutionTest 
{
    @Test
    public void testGenerateCave() {
        final String input = "#######\n#.G.E.#\n#E.G.E#\n#.G.E.#\n#######";
        final Cave cave = Solution.constructCaveFloorMap(input);
        assertEquals(7, cave.entities.size());
        assertEquals(35, cave.world.size());
        assertEquals(7, cave.width);
        assertEquals(5, cave.height);
    }
}
