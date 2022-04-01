package aoc;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public class Cave {
    final int width;
    final int height;
    Map<Point, Tile> world;
    List<Entity> entities;

    public static Cave fromString(String input) {
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

    public boolean isFloor(Point p) {
        return world.getOrDefault(p, Tile.Wall) == Tile.Floor;
    }

    public Optional<Entity> getEntityAtPosition(int x, int y) {
        return entities
                .stream()
                .filter((Entity e) -> e.position.equals(new Point(x, y)))
                .findFirst();
    }

    @Override
    public String toString() {
        Map<Point, EntityType> ePos = new HashMap<>();
        for (var e : entities) {
            ePos.put(e.position, e.type);
        }

        StringBuilder out = new StringBuilder();
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                var point = new Point(x, y);
                out.append(ePos.containsKey(point) ? ePos.get(point).label : world.get(point).label);
            }
            if (y < height - 1) {
                out.append('\n');
            }
        }
        return out.toString();
    }
}
