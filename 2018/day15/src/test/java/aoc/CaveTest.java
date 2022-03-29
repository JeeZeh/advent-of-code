package aoc;

import static org.junit.Assert.assertEquals;
import org.junit.Test;

public class CaveTest {
    @Test
    public void testFromString() {
        final String input = "#######\n#.G.E.#\n#E.G.E#\n#.G.E.#\n#######";
        final Cave cave = Cave.fromString(input);
        assertEquals(7, cave.entities.size());
        assertEquals(35, cave.world.size());
        assertEquals(7, cave.width);
        assertEquals(5, cave.height);
    }

    @Test
    public void testToString() {
        final String input = "#######\n#.G.E.#\n#E.G.E#\n#.G.E.#\n#######";
        final String result = Cave.fromString(input).toString();
        assertEquals(input, result);
    }
}
