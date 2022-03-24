package aoc;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

enum Tile {
    Wall, Floor
}

enum Entity {
    Goblin, Elf
}

public class Solution {
    public static void main(String[] args) {
        // Get input
        String input = "#########\n#G..G..G#\n#.......#\n#.......#\n#G..E..G#\n#.......#\n#.......#\n#G..G..G#\n#########";
        // Init cave
        Cave cave = constructCaveFloorMap(input);
        
        // Start round
        boolean playing = true;
        while (playing) {
            // Gather order of entities
            List<Entity> turnReadingOrder = cave.entities.entrySet().stream().sorted((a, b) -> a.getKey() < b.getKey())
        }

        // For each entity
            //      Get all targets
            //      Any in range? 
                //      Check UDLR
                //      Attack if enemy
                //      Resolve ties with reading order
            //      Else Move
                //      BFS outwards to find the nearest destinatios
                    //      Cannot walk through entities or walls
                    //      Only look at current state of the world
                    //      Attack location = UDLR adjacent positions
                    //      Use Manhattan Distance
                //      Always move towards chosen destination in the order of ULRD
                
    }

    public static Cave constructCaveFloorMap(String input) {
        Map<Point, Tile> world = new HashMap<>();
        Map<Point, Entity> entities = new HashMap<>();
        List<String> lines = input.lines().toList();
        for (int y = 0; y < lines.size(); y++) {
            String line = lines.get(y);
            for (int x = 0; x < line.length(); x++) {
                char c = line.charAt(x);
                Tile tile = c == '#' ? Tile.Wall : Tile.Floor;
                world.put(new Point(x, y), tile);
                if (c == 'G' || c == 'E') {
                    entities.put(new Point(x, y), c == 'G' ? Entity.Goblin : Entity.Elf);
                }
            }
        }

        return new Cave(lines.get(0).length(), lines.size(), world, entities);
    }
}
