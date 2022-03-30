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

    @Test
    public void testAttackInRange() {
        // #######
        // #...G.#
        // #...EG#
        // #.....#
        // #######
        String starting = "#######\n#...G.#\n#...EG#\n#.....#\n#######";
        Game game = new Game(Cave.fromString(starting), false);

        // final Entity expectedTarget = game.cave.entities
        // .stream()
        // .filter((Entity e) -> e.position.equals(new Point(4, 1)))
        // .findFirst()
        // .get();

        game.step();

        // Every target should
        var entities = game.cave.entities.stream().sorted().toList();
        assertEquals(entities.get(0).hp, 197);
        assertEquals(entities.get(1).hp, 194);
        assertEquals(entities.get(2).hp, 200);
        assertEquals(starting, game.cave.toString());
    }

    @Test
    public void testDeadTargetsAreRemovedFromPlay() {
        // #######
        // #...G.#
        // #...EG#
        // #.....#
        // #######
        String starting = "#######\n#...G.#\n#...EG#\n#.....#\n#######";
        Game game = new Game(Cave.fromString(starting), false);

        game.cave.getEntityAtPosition(4, 2).get().hp = 4;
        game.step();

        // Every target should
        assertEquals(game.cave.entities.size(), 2);
        assertEquals("#######\n#...G.#\n#....G#\n#.....#\n#######", game.cave.toString());
    }
}

