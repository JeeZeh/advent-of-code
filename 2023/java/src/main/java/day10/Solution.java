package day10;

import day10.Solution.Tiles.Step;
import java.io.IOException;
import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day10/example2.txt").toList();
    var tiles = Tiles.fromLines(lines);
    var distances = tiles.getDistances(tiles.start);
    System.out.println(distances);
    int partOne = distances.values().stream().max(Integer::compareTo).get();

    System.out.println(STR."Part 1: \{partOne}");
  }


  public record Tiles(Tile[][] elements, Pos start) implements Grid<Tile> {

    public record Step(Pos pos, int distance) {

    }

    public Map<Pos, Integer> getDistances(Pos start) {
      final Deque<Step> queue = new ArrayDeque<>();
      final Map<Pos, Integer> distances = new HashMap<>();
      distances.put(start, 0);

      // Find next pipe locations
      nextSteps(start).forEach(queue::add);

      while (!queue.isEmpty()) {
        Step next = queue.poll();
        if (!distances.containsKey(next.pos)) {
          distances.put(next.pos, next.distance);

          nextSteps(next.pos).filter(step -> !distances.containsKey(step.pos))
              .map(step -> new Step(step.pos, step.distance + next.distance)).forEach(queue::add);
        }
      }

      return distances;
    }

    Stream<Step> nextSteps(Pos pos) {
      // Find next pipe locations
      return this.surroundingPositions(pos).map(p -> this.tryMove(pos, p))
          .filter(step -> step.distance != 0);
    }

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

    public Step tryMove(Pos from, Pos to) {
      // Not a valid destination
      if (to.y() < 0) {
        return new Step(from, 0);
      }

      Optional<Direction> maybeApproach = Direction.getDir(from, to);

      // Not a valid approach direction
      if (maybeApproach.isEmpty()) {
        return new Step(from, 0);
      }

      // Can't enter this tile from origin
      Direction approach = maybeApproach.get();
      Tile tileType = this.elements[to.y()][to.x()];
      if (!tileType.canEnter(approach)) {
        return new Step(from, 0);
      }

      return tileType.translate(from, approach);
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

    public Step translate(Pos origin, Direction approach) {
      return switch (this) {
        case VERT -> new Step(approach == Direction.UP ? origin.add(0, -1) : origin.add(0, 1), 1);
        case HORIZ ->
            new Step(approach == Direction.LEFT ? origin.add(-1, 0) : origin.add(1, 0), 1);
        case CORNER_NE ->
            new Step(approach == Direction.DOWN ? origin.add(1, 1) : origin.add(-1, -1), 2);
        case CORNER_NW ->
            new Step(approach == Direction.DOWN ? origin.add(-1, 1) : origin.add(1, -1), 2);
        case CORNER_SW ->
            new Step(approach == Direction.RIGHT ? origin.add(1, 1) : origin.add(-1, -1), 2);
        case CORNER_SE ->
            new Step(approach == Direction.LEFT ? origin.add(-1, 1) : origin.add(1, -1), 12);
        case START -> new Step(origin, 0);
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
