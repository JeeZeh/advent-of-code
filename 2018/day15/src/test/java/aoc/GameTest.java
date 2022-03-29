package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import org.junit.Test;

public class GameTest {
    @Test
    public void testMovementSteps() {
        String start =
                "#########\n#G..G..G#\n#.......#\n#.......#\n#G..E..G#\n#.......#\n#.......#\n#G..G..G#\n#########";
        Game game = new Game(Cave.fromString(start), true);
        assertEquals(start, game.toString());

        String roundOne =
                "#########\n#.G...G.#\n#...G...#\n#...E..G#\n#.G.....#\n#.......#\n#G..G..G#\n#.......#\n#########";
        assertEquals(roundOne, game.step().get().toString());

        String roundTwo =
                "#########\n#..G.G..#\n#...G...#\n#.G.E.G.#\n#.......#\n#G..G..G#\n#.......#\n#.......#\n#########";
        assertEquals(roundTwo, game.step().get().toString());

        String roundThree =
                "#########\n#.......#\n#..GGG..#\n#..GEG..#\n#G..G...#\n#......G#\n#.......#\n#.......#\n#########";
        assertEquals(roundThree, game.step().get().toString());

        assertTrue(game.step().isEmpty());
    }

    public static String difference(String str1, String str2) {
        if (str1 == null) {
            return str2;
        }
        if (str2 == null) {
            return str1;
        }
        int at = indexOfDifference(str1, str2);
        if (at == -1) {
            return "";
        }
        return str2.substring(at);
    }

    public static int indexOfDifference(CharSequence cs1, CharSequence cs2) {
        if (cs1 == cs2) {
            return -1;
        }
        if (cs1 == null || cs2 == null) {
            return 0;
        }
        int i;
        for (i = 0; i < cs1.length() && i < cs2.length(); ++i) {
            if (cs1.charAt(i) != cs2.charAt(i)) {
                break;
            }
        }
        if (i < cs2.length() || i < cs1.length()) {
            return i;
        }
        return -1;
    }
}

