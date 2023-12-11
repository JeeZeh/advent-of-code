package day10;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day10/input.txt").toList();
    var tiles = Tiles.fromLines(lines);
    var distances = tiles.getDistances(tiles.start);
    int partOne = distances.values().stream().max(Integer::compareTo).get();
    var enclosed = tiles.findEnclosed(distances.keySet());
    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{enclosed.size()}");
  }


  public record Tiles(Tile[][] elements, Pos start) implements Grid<Tile> {

    public record Step(Pos pos, int distance) {

    }

    Pos getWalkStart(Set<Pos> loop) {
      for (int y = 0; y < elements.length; y++) {
        var row = elements[y];
        for (int x = 0; x < row.length; x++) {
          var probe = new Pos(x, y);
          if (loop.contains(probe)) {
            return probe;
          }
        }
      }
      throw new IllegalStateException("Could not find starting position");
    }

    Direction getCheckDir(Direction heading) {
      return switch (heading) {
        case UP -> Direction.RIGHT;
        case DOWN -> Direction.LEFT;
        case LEFT -> Direction.UP;
        case RIGHT -> Direction.DOWN;
        default -> throw new IllegalStateException(STR."Unexpected value: \{heading}");
      };
    }

    Set<Pos> findEnclosed(Set<Pos> loop) {
      Set<Pos> enclosed = new HashSet<>();
      Pos start = getWalkStart(loop);
      Pos loc = start.add(0, 0); // Copy

      Direction heading = Direction.getDir(start, nextSteps(start).findFirst().get().pos).get();
      Direction checkDir = getCheckDir(heading);

      do {
        var check = checkDir.transpose(loc);
        if (!loop.contains(check) && !enclosed.contains(check)) {
          enclosed.addAll(findNonLoop(check, loop));
        }

        final var prevLoc = loc;
        loc = heading.transpose(loc);
        heading = switch (elements[loc.y()][loc.x()]) {
          case CORNER_NE -> heading == Direction.DOWN ? Direction.RIGHT : Direction.UP;
          case CORNER_NW -> heading == Direction.DOWN ? Direction.LEFT : Direction.UP;
          case CORNER_SW -> heading == Direction.UP ? Direction.LEFT : Direction.DOWN;
          case CORNER_SE -> heading == Direction.UP ? Direction.RIGHT : Direction.DOWN;
          case START -> Direction.getDir(loc,
              // Find the direction we must be hading in if we hit the start
              nextSteps(loc).filter(step -> !step.pos.equals(prevLoc)).findFirst().get().pos).get();
          default -> heading;
        };
        checkDir = getCheckDir(heading);
      } while (!loc.equals(start));

      return enclosed;
    }

    public Map<Pos, Integer> getDistances(Pos start) {
      final Deque<Step> queue = new ArrayDeque<>();
      final Map<Pos, Integer> distances = new HashMap<>();
      distances.put(start, 0);

      // Find next pipe locations
      nextSteps(start).forEach(queue::add);

      while (!queue.isEmpty()) {
        Step curr = queue.poll();
        if (!distances.containsKey(curr.pos)) {
          distances.put(curr.pos, curr.distance);

          nextSteps(curr.pos).filter(next -> !distances.containsKey(next.pos))
              .map(next -> new Step(next.pos, 1 + curr.distance)).forEach(queue::add);
        }
      }

      return distances;
    }

    public Set<Pos> findNonLoop(Pos start, Set<Pos> loop) {
      final Deque<Pos> queue = new ArrayDeque<>();
      final Set<Pos> seen = new HashSet<>();
      queue.add(start);

      while (!queue.isEmpty()) {
        Pos curr = queue.poll();
        if (!seen.contains(curr)) {
          seen.add(curr);
          surroundingPositions(curr).filter(next -> !loop.contains(next)).forEach(queue::add);
        }
      }

      return seen;
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
      Optional<Direction> maybeApproach = Direction.getDir(from, to);

      // Not a valid approach direction
      if (maybeApproach.isEmpty()) {
        return new Step(from, 0);
      }
      Direction approach = maybeApproach.get();

      Tile sourceType = this.elements[from.y()][from.x()];
      if (!sourceType.canTraverse(approach.invert())) {
        return new Step(from, 0);
      }

      // Can't enter this tile from origin
      Tile targetType = this.elements[to.y()][to.x()];
      if (!targetType.canTraverse(approach)) {
        return new Step(from, 0);
      }

      return new Step(to, 1);
    }
  }

  public enum Tile {
    VERT, HORIZ, CORNER_NE, CORNER_NW, CORNER_SW, CORNER_SE, GROUND, START;

    public boolean canTraverse(Direction approach) {
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
