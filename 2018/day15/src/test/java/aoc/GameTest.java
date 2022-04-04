package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import java.util.Optional;
import org.junit.Test;

public class GameTest {
        @Test
        public void testMovementSteps() throws ElfDiedException {
                String start =
                                "#########\n#G..G..G#\n#.......#\n#.......#\n#G..E..G#\n#.......#\n#.......#\n#G..G..G#\n#########";
                Game game = new Game(Cave.fromString(start), true, false);
                assertEquals(start, game.toString());

                String roundOne =
                                "#########\n#.G...G.#\n#...G...#\n#...E..G#\n#.G.....#\n#.......#\n#G..G..G#\n#.......#\n#########";
                assertEquals(roundOne, game.nextRound().get().toString());

                String roundTwo =
                                "#########\n#..G.G..#\n#...G...#\n#.G.E.G.#\n#.......#\n#G..G..G#\n#.......#\n#.......#\n#########";
                assertEquals(roundTwo, game.nextRound().get().toString());

                String roundThree =
                                "#########\n#.......#\n#..GGG..#\n#..GEG..#\n#G..G...#\n#......G#\n#.......#\n#.......#\n#########";
                assertEquals(roundThree, game.nextRound().get().toString());
        }

        @Test
        public void testAttackInRange() throws ElfDiedException {
                // #######
                // #...G.#
                // #...EG#
                // #.....#
                // #######
                String starting = "#######\n#...G.#\n#...EG#\n#.....#\n#######";
                Game game = new Game(Cave.fromString(starting), false, false);

                // final Entity expectedTarget = game.cave.entities
                // .stream()
                // .filter((Entity e) -> e.position.equals(new Point(4, 1)))
                // .findFirst()
                // .get();

                game.nextRound();

                // Every target should have taken damage except for the one at 5, 2
                assertEquals(game.cave.getEntityAtPosition(4, 1).get().hp, 197);
                assertEquals(game.cave.getEntityAtPosition(4, 2).get().hp, 194);
                assertEquals(game.cave.getEntityAtPosition(5, 2).get().hp, 200);
                assertEquals(starting, game.cave.toString());
        }

        @Test
        public void testDeadTargetsAreRemovedFromPlay() throws ElfDiedException {
                // #######
                // #...G.#
                // #...EG#
                // #.....#
                // #######
                String starting = "#######\n#...G.#\n#...EG#\n#.....#\n#######";
                Game game = new Game(Cave.fromString(starting), false, false);

                game.cave.getEntityAtPosition(4, 2).get().hp = 4;
                game.nextRound();

                // Check that one target died
                assertEquals(game.cave.entities.size(), 2);
                assertTrue(game.cave.getEntityAtPosition(4, 2).isEmpty());
                assertEquals("#######\n#...G.#\n#....G#\n#.....#\n#######", game.cave.toString());
        }

        @Test
        public void testExampleZero() throws ElfDiedException {
                // Initial
                // #######
                // #.G...# G(200)
                // #...EG# E(200), G(200)
                // #.#.#G# G(200)
                // #..G#E# G(200), E(200)
                // #.....#
                // #######
                String initial = "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######";
                Game game = new Game(Cave.fromString(initial), false, false);

                // After 1 step
                // #######
                // #..G..# G(200)
                // #...EG# E(197), G(197)
                // #.#G#G# G(200), G(197)
                // #...#E# E(197)
                // #.....#
                // #######
                game.nextRound();
                assertEquals(game.cave.getEntityAtPosition(3, 1).get().hp, 200);
                assertEquals(game.cave.getEntityAtPosition(4, 2).get().hp, 197);
                assertEquals(game.cave.getEntityAtPosition(5, 2).get().hp, 197);
                assertEquals(game.cave.getEntityAtPosition(3, 3).get().hp, 200);
                assertEquals(game.cave.getEntityAtPosition(5, 2).get().hp, 197);

                for (int i = 0; i < 22; i++) {
                        game.nextRound();
                }

                // After 23 steps
                // #######
                // #...G.# G(200)
                // #..G.G# G(200), G(131)
                // #.#.#G# G(131)
                // #...#E# E(131)
                // #.....#
                // #######
                assertEquals(game.cave.getEntityAtPosition(4, 1).get().hp, 200);
                assertEquals(game.cave.getEntityAtPosition(3, 2).get().hp, 200);
                assertEquals(game.cave.getEntityAtPosition(5, 2).get().hp, 131);
                assertEquals(game.cave.getEntityAtPosition(5, 3).get().hp, 131);
                assertEquals(game.cave.getEntityAtPosition(5, 4).get().hp, 131);
        }

        @Test
        public void testGameFinishes() throws ElfDiedException {
                // Initial
                // #######
                // #.G...# G(200)
                // #...EG# E(200), G(200)
                // #.#.#G# G(200)
                // #..G#E# G(200), E(200)
                // #.....#
                // #######
                String initial = "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######";
                Game game = new Game(Cave.fromString(initial), false, false).play(false);

                assertEquals(47, game.getSummary().rounds);
                assertEquals(590, game.getSummary().totalHpRemaining);
                assertEquals(27730, game.getSummary().outcome);

                String summaryString =
                                "Combat ends after 47 full rounds\nGoblins win with 590 total hit points left\nOutcome: 47 * 590 = 27730";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleOne() throws ElfDiedException {
                Game game = new Game(Cave.fromString("#######\n#G..#E#\n#E#E.E#\n#G.##.#\n#...#E#\n#...E.#\n#######"),
                                false, false)
                                                .play(false);
                String summaryString =
                                "Combat ends after 37 full rounds\nElves win with 982 total hit points left\nOutcome: 37 * 982 = 36334";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleTwo() throws ElfDiedException {
                Game game = new Game(Cave.fromString("#######\n#E..EG#\n#.#G.E#\n#E.##E#\n#G..#.#\n#..E#.#\n#######"),
                                false, false)
                                                .play(false);
                String summaryString =
                                "Combat ends after 46 full rounds\nElves win with 859 total hit points left\nOutcome: 46 * 859 = 39514";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleThree() throws ElfDiedException {
                Game game = new Game(Cave.fromString("#######\n#E.G#.#\n#.#G..#\n#G.#.G#\n#G..#.#\n#...E.#\n#######"),
                                false, false)
                                                .play(false);
                String summaryString =
                                "Combat ends after 35 full rounds\nGoblins win with 793 total hit points left\nOutcome: 35 * 793 = 27755";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleFour() throws ElfDiedException {
                Game game = new Game(Cave.fromString("#######\n#.E...#\n#.#..G#\n#.###.#\n#E#G#G#\n#...#G#\n#######"),
                                false, false)
                                                .play(false);
                String summaryString =
                                "Combat ends after 54 full rounds\nGoblins win with 536 total hit points left\nOutcome: 54 * 536 = 28944";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleFive() throws ElfDiedException {
                Game game = new Game(Cave.fromString(
                                "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########"),
                                false, false)
                                                .play(false);
                String summaryString =
                                "Combat ends after 20 full rounds\nGoblins win with 937 total hit points left\nOutcome: 20 * 937 = 18740";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testExampleFivePartTwo() throws ElfDiedException {
                String input = "#########\n#G......#\n#.E.#...#\n#..##..G#\n#...##..#\n#...#...#\n#.G...G.#\n#.....G.#\n#########";

                Game game = null;
                for (int ap = 4; ap < 100; ap++) {
                        game = new Game(Cave.fromString(input), false, true);
                        var tryAp = ap;
                        game.cave.entities.stream().filter((Entity e) -> e.type == EntityType.Elf)
                                        .forEach((Entity e) -> e.AP = tryAp);
                        try {
                                game.play(false);
                                break;
                        } catch (ElfDiedException e) {
                                continue;
                        }
                }
                String summaryString =
                                "Combat ends after 30 full rounds\nElves win with 38 total hit points left\nOutcome: 30 * 38 = 1140";
                assertEquals(summaryString, game.getSummary().toString());
        }

        @Test
        public void testMinimumHpForElvesToSurvive() {
                Game game = null;
                int ap = 4;
                while (true) {
                        game = new Game(Cave.fromString(
                                        "#######\n#.G...#\n#...EG#\n#.#.#G#\n#..G#E#\n#.....#\n#######"), false,
                                        true);
                        var tryAp = ap;
                        game.cave.entities.stream().filter((Entity e) -> e.type == EntityType.Elf)
                                        .forEach((Entity e) -> e.AP = tryAp);
                        try {
                                game.play(false);
                                break;
                        } catch (ElfDiedException e) {
                                ap++;
                                continue;
                        }
                }

                assertEquals(ap, 15);
        }
}

