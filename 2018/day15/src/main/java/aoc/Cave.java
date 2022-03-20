package aoc;

import java.util.Map;

import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public class Cave {
    final int width;
    final int height;
    Map<Point, Tile> world;
    Map<Point, Entity> entities;

    @Override
    public String toString() {
        return "";
    }
}
