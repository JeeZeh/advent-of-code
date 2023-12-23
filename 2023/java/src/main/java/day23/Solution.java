package day23;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.PriorityQueue;
import java.util.Set;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  private static final List<Direction> DIRECTIONS = List.of(Direction.UP, Direction.RIGHT,
      Direction.DOWN, Direction.LEFT);

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day23/example.txt").toList();
    var trail = Trail.fromLines(lines);
    Pos start = new Pos(1, 0);
    Pos goal = new Pos(trail.width() - 2, trail.height() - 1);
    long partOne = trail.longestWalk(start, goal);
    long partTwo = 0;

    /**
     * Keep directions in List. When hitting slope, shuffle the direction so that you always consider
     * a downward movement, followed by that direction. Take a step and reset the index of the direction
     * you consider each time.
     */
  }

  public record Trail(List<List<Tile>> elements) implements Grid<Tile> {

    record Step(Pos pos, Direction heading, boolean mustFall, Set<Pos> seen) {

    }

    public long longestWalk(Pos start, Pos end) {
      PriorityQueue<Step> nextSteps = new PriorityQueue<>(
          Comparator.comparing(s -> -s.seen.size()));

      nextSteps.add(new Step(start, Direction.DOWN, false, new HashSet<>()));

      long highest = Long.MIN_VALUE;

      while (!nextSteps.isEmpty()) {
        var curr = nextSteps.poll();
        if (curr.pos.equals(end)) {
          highest = Math.max(curr.seen.size(), highest);
        } else {
          nextSteps.addAll(getNextSteps(curr));
        }
      }
      return highest;
    }

    List<Step> getNextSteps(Step curr) {
      List<Step> nextSteps = new ArrayList<>();
      Pos nextPos;
      Set<Pos> newSeen = new HashSet<>(curr.seen);

      // Check down
      Pos down = Direction.DOWN.transpose(curr.pos);
      if (isWithin(down)) {
        var tile = get(down);
        if (tile != Tile.FOREST) {
          nextPos = down;
          var heading = switch (tile) {
            case SLOPE_UP -> Direction.UP;
            case SLOPE_RIGHT -> Direction.RIGHT;
            case SLOPE_DOWN -> Direction.DOWN;
            case SLOPE_LEFT -> Direction.LEFT;
            default -> curr.heading;
          };
          if (!curr.seen.contains(down)) {
            newSeen.add(down);
            return List.of(new Step(nextPos, heading, tile != Tile.PATH, newSeen));
          }
        }
      }

      // Check
      Pos walk = curr.heading.transpose(curr.pos);
      if (isWithin(walk)) {
        var tile = get(walk);
        if (tile != Tile.FOREST) {
          nextPos = walk;
          var heading = switch (tile) {
            case SLOPE_UP -> Direction.UP;
            case SLOPE_RIGHT -> Direction.RIGHT;
            case SLOPE_DOWN -> Direction.DOWN;
            case SLOPE_LEFT -> Direction.LEFT;
            default -> curr.heading;
          };
          if (!curr.seen.contains(down)) {
            newSeen.add(walk);
            return List.of(new Step(nextPos, heading, tile != Tile.PATH, newSeen));
          }
        }
      }

      // Check remaining
      return DIRECTIONS.stream().filter(d -> d != Direction.DOWN && d != curr.heading).map(dir -> {
        Pos newPos = dir.transpose(curr.pos);
        if (isWithin(newPos)) {
          var tile = get(newPos);
          if (tile != Tile.FOREST) {
            var heading = switch (tile) {
              case SLOPE_UP -> Direction.UP;
              case SLOPE_RIGHT -> Direction.RIGHT;
              case SLOPE_DOWN -> Direction.DOWN;
              case SLOPE_LEFT -> Direction.LEFT;
              default -> curr.heading;
            };
            if (!curr.seen.contains(down)) {
              newSeen.add(newPos);
              return new Step(newPos, heading, tile != Tile.PATH, newSeen);
            }
          }
        }
        return null;
      }).filter(Objects::nonNull).toList();
    }

    static Trail fromLines(List<String> lines) {
      return new Trail(
          lines.stream().map(line -> line.chars().mapToObj(c -> Tile.fromChar((char) c)).toList())
              .toList());
    }
  }


  public enum Tile {
    PATH, FOREST, SLOPE_UP, SLOPE_RIGHT, SLOPE_DOWN, SLOPE_LEFT;

    static Tile fromChar(char c) {
      return switch (c) {
        case '.' -> PATH;
        case '#' -> FOREST;
        case '^' -> SLOPE_UP;
        case '>' -> SLOPE_RIGHT;
        case 'v' -> SLOPE_DOWN;
        case '<' -> SLOPE_LEFT;
        default -> throw new IllegalStateException(STR."Unexpected value: \{c}");
      };
    }

    @Override
    public String toString() {
      return switch (this) {
        case PATH -> ".";
        case FOREST -> "#";
        case SLOPE_UP -> "^";
        case SLOPE_RIGHT -> ">";
        case SLOPE_DOWN -> "v";
        case SLOPE_LEFT -> "<";
      };
    }
  }
}
