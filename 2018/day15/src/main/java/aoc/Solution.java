package aoc;

import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.ListIterator;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Collectors;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.Accessors;

enum Tile {
    Wall, Floor
}

enum Entity {
    Goblin, Elf
}

public class Solution {
    public static void main(String[] args) {

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
