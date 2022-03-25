package aoc;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;



public class Solution {
    public static void main(String[] args) {
        // Get input
        String input =
                "#########\n#G..G..G#\n#.......#\n#.......#\n#G..E..G#\n#.......#\n#.......#\n#G..G..G#\n#########";
        Cave cave = constructCaveFloorMap(input);
        play(cave);
    }

    public static void play(Cave cave) {
        // Start round
        var playing = true;
        while (playing) {
            // Gather order of entities
            playing = false;

            for (final Entity takingTurn : cave.entities.stream().sorted().toList()) {
                var targets = cave.entities.stream().filter((Entity e) -> e.type != takingTurn.type).toList();
                if (targets.size() == 0) {
                    return;
                }

                var toAttack = takingTurn.findTargetInRange(cave.entities);
                if (toAttack.isPresent()) {
                    // Attack
                } else {
                    // Move

                    // BFS outwards to find the nearest destinatios
                    // Cannot walk through entities or walls
                    // Only look at current state of the world
                    // Attack location = UDLR adjacent positions
                    // Use Manhattan Distance
                    // Always move towards chosen destination in the order of ULRD
                }
            }
        }
    }

    public static Cave constructCaveFloorMap(String input) {
        Map<Point, Tile> world = new HashMap<>();
        List<Entity> entities = new ArrayList<>();
        var lines = input.lines().toList();
        for (int y = 0; y < lines.size(); y++) {
            String line = lines.get(y);
            for (int x = 0; x < line.length(); x++) {
                char c = line.charAt(x);
                Tile tile = c == '#' ? Tile.Wall : Tile.Floor;
                world.put(new Point(x, y), tile);
                if (c == 'G') {
                    entities.add(new Entity(EntityType.Goblin, new Point(x, y)));
                } else if (c == 'E') {
                    entities.add(new Entity(EntityType.Elf, new Point(x, y)));
                }
            }
        }

        return new Cave(lines.get(0).length(), lines.size(), world, entities);
    }
}
