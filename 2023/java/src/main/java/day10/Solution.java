package day10;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day10/example1.txt").toList();
    Tiles tiles = Tiles.fromLines(lines);
    List<Pos> startingTiles = tiles.start.neighbours()
        .map(p -> tiles.elements[p.y()][p.x()].translate(tiles.start, p))
        .filter(newPos -> newPos != tiles.start).toList();
    System.out.println(STR."Starting tiles: \{Arrays.toString(startingTiles.toArray())}");
  }


  public record Tiles(Tile[][] elements, Pos start) implements Grid<Tile> {

    static Tiles fromLines(List<String> lines) {
      int height = lines.size();
      int width = lines.getFirst().length();
      Tile[][] elements = new Tile[height][width];
      Pos start = null;

      for (int y = 0; y < lines.size(); y++) {
        String row = lines.get(y);
        for (int x = 0; x < row.length(); x++) {
          elements[y][x] = Tile.fromChar(row.charAt(x));
          if (elements[y][x] == Tile.START) {
            start = new Pos(x, y);
          }
        }
      }

      if (start == null) {
        throw new RuntimeException("Failed to locate starting position");
      }

      return new Tiles(elements, start);
    }
  }

  public enum Tile {
    VERT, HORIZ, CORNER_NE, CORNER_NW, CORNER_SW, CORNER_SE, GROUND, START;

    public boolean canEnter(Direction approach) {
      return switch (this) {
        case VERT -> approach == Direction.UP || approach == Direction.DOWN;
        case HORIZ -> approach == Direction.LEFT || approach == Direction.RIGHT;
        case CORNER_NE -> approach == Direction.DOWN || approach == Direction.LEFT;
        case CORNER_NW -> approach == Direction.DOWN || approach == Direction.RIGHT;
        case CORNER_SW -> approach == Direction.UP || approach == Direction.RIGHT;
        case CORNER_SE -> approach == Direction.UP || approach == Direction.LEFT;
        case GROUND -> false;
        case START -> true;
      };
    }

    public boolean isPipe() {
      return this != GROUND && this != START;
    }

    public Pos translate(Pos origin, Pos tileLoc) {
      Optional<Direction> maybeMovement = Direction.getDir(origin, tileLoc);
      if (maybeMovement.isEmpty()) {
        return origin;
      }

      Direction movement = maybeMovement.get();
      if (!this.canEnter(movement)) {
        return origin;
      }

      return switch (this) {
        case VERT -> movement == Direction.UP ? origin.add(0, -1) : origin.add(0, 1);
        case HORIZ -> movement == Direction.LEFT ? origin.add(-1, 0) : origin.add(1, 0);
        case CORNER_NE -> movement == Direction.DOWN ? origin.add(1, 1) : origin.add(-1, -1);
        case CORNER_NW -> movement == Direction.DOWN ? origin.add(-1, 1) : origin.add(1, -1);
        case CORNER_SW -> movement == Direction.RIGHT ? origin.add(1, 1) : origin.add(-1, -1);
        case CORNER_SE -> movement == Direction.LEFT ? origin.add(-1, 1) : origin.add(1, -1);
        case START -> origin;
        default -> throw new IllegalStateException(STR."Unexpected value: \{this}");
      };
    }

    public static Tile fromChar(char c) {
      return switch (c) {
        case '|' -> Tile.VERT;
        case '-' -> Tile.HORIZ;
        case 'L' -> Tile.CORNER_NE;
        case 'J' -> Tile.CORNER_NW;
        case '7' -> Tile.CORNER_SW;
        case 'F' -> Tile.CORNER_SE;
        case '.' -> Tile.GROUND;
        case 'S' -> Tile.START;
        default -> throw new IllegalStateException(STR."Unexpected value: \{c}");
      };
    }

    @Override
    public String toString() {
      return switch (this) {
        case VERT -> "|";
        case HORIZ -> "-";
        case CORNER_NE -> "L";
        case CORNER_NW -> "J";
        case CORNER_SW -> "7";
        case CORNER_SE -> "F";
        case GROUND -> ".";
        case START -> "S";
      };
    }
  }

}
