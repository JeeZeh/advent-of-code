package day16;

import day16.Solution.Contraption.Light;
import day16.Solution.Contraption.Tile;
import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {


  public static void main(String[] args) throws IOException {
    Contraption contraption = Contraption.fromLines(Input.lines("day16/input.txt").toList());
    var energized = contraption.fireBeam(new Light(new Pos(-1, 0), Direction.RIGHT));
    System.out.println(STR."Part 1: \{energized.size()}");

    int maxDown = IntStream.range(0, contraption.width())
        .map(x -> contraption.fireBeam(new Light(new Pos(x, -1), Direction.DOWN)).size()).max()
        .getAsInt();
    int maxUp = IntStream.range(0, contraption.width()).map(
            x -> contraption.fireBeam(new Light(new Pos(x, contraption.height()), Direction.UP)).size())
        .max().getAsInt();
    int maxRight = IntStream.range(0, contraption.height())
        .map(y -> contraption.fireBeam(new Light(new Pos(-1, y), Direction.RIGHT)).size()).max()
        .getAsInt();
    int maxLeft = IntStream.range(0, contraption.height()).map(
        y -> contraption.fireBeam(new Light(new Pos(contraption.width(), y), Direction.LEFT))
            .size()).max().getAsInt();

    int partTwo = IntStream.of(maxUp, maxLeft, maxRight, maxDown).max().getAsInt();
    System.out.println(STR."Part 2: \{partTwo}");
  }


  record Contraption(List<List<Tile>> elements) implements Grid<Tile> {

    record Light(Pos pos, Direction dir) {

    }

    public String showEnergized(Set<Pos> energized) {
      List<List<Tile>> grid = new ArrayList<>();
      for (int y = 0; y < elements.size(); y++) {
        var row = new ArrayList<>(elements.get(y));
        grid.add(row);
        for (int x = 0; x < row.size(); x++) {
          if (energized.contains(new Pos(x, y))) {
            grid.get(y).set(x, Tile.ENERGY);
          }
        }

      }
//      energized.forEach(p -> grid[p.y()][p.x()] = Tile.ENERGY);

      return new Contraption(grid).asString();
    }

    public Set<Pos> fireBeam(Light start) {
      ArrayDeque<Light> nextBeams = new ArrayDeque<>();
      Set<Pos> energized = new HashSet<>();
      Set<Light> seen = new HashSet<>();
      deflect(start, start.dir.transpose(start.pos)).filter(light -> isWithin(light.pos))
          .forEach(nextBeams::add);

      while (!nextBeams.isEmpty()) {
//        System.out.println(showEnergized(energized));
        var curr = nextBeams.poll();
        if (seen.contains(curr)) {
          continue;
        } else {
          seen.add(curr);
        }
        energized.add(curr.pos);
        var nextPos = curr.dir.transpose(curr.pos);
        if (isWithin(nextPos)) {
          energized.add(nextPos);
          deflect(curr, nextPos).filter(light -> isWithin(light.pos)).forEach(nextBeams::add);
        }
      }

      return energized;
    }

    static Contraption fromLines(List<String> lines) {
      return new Contraption(
          lines.stream().map(line -> line.chars().mapToObj(c -> Tile.fromChar((char) c)).toList())
              .toList());
    }

    Stream<Light> deflect(Light light, Pos to) {
      var targetTile = this.get(to);
      if (targetTile == Tile.EMPTY) {
        return Stream.of(new Light(to, light.dir));
      }

      var approach = Pos.Direction.getDir(light.pos, to);
      if (approach.isEmpty()) {
        return Stream.of();
      }

      if (targetTile == Tile.MIRROR_RIGHT) {
        return switch (approach.get()) {
          case UP -> Stream.of(new Light(to, Direction.RIGHT));
          case DOWN -> Stream.of(new Light(to, Direction.LEFT));
          case LEFT -> Stream.of(new Light(to, Direction.DOWN));
          case RIGHT -> Stream.of(new Light(to, Direction.UP));
          default -> throw new IllegalStateException(STR."Unexpected approach: \{approach}");
        };
      }
      if (targetTile == Tile.MIRROR_LEFT) {
        return switch (approach.get()) {
          case UP -> Stream.of(new Light(to, Direction.LEFT));
          case DOWN -> Stream.of(new Light(to, Direction.RIGHT));
          case LEFT -> Stream.of(new Light(to, Direction.UP));
          case RIGHT -> Stream.of(new Light(to, Direction.DOWN));
          default -> throw new IllegalStateException(STR."Unexpected approach: \{approach}");
        };
      }
      if (targetTile == Tile.SPLITTER_HORIZ) {
        return switch (approach.get()) {
          case LEFT, RIGHT -> Stream.of(new Light(to, light.dir));
          case UP, DOWN -> Stream.of(new Light(to, Direction.LEFT), new Light(to, Direction.RIGHT));
          default -> throw new IllegalStateException(STR."Unexpected approach: \{approach}");
        };
      }
      if (targetTile == Tile.SPLITTER_VERT) {
        return switch (approach.get()) {
          case UP, DOWN -> Stream.of(new Light(to, light.dir));
          case LEFT, RIGHT -> Stream.of(new Light(to, Direction.UP), new Light(to, Direction.DOWN));
          default -> throw new IllegalStateException(STR."Unexpected approach: \{approach}");
        };
      }

      throw new IllegalStateException("Could not determine deflection");
    }

    public enum Tile {
      EMPTY, MIRROR_RIGHT, MIRROR_LEFT, SPLITTER_VERT, SPLITTER_HORIZ, ENERGY;

      static Tile fromChar(char c) {
        return switch (c) {
          case '.' -> EMPTY;
          case '/' -> MIRROR_RIGHT;
          case '\\' -> MIRROR_LEFT;
          case '-' -> SPLITTER_HORIZ;
          case '|' -> SPLITTER_VERT;
          case '#' -> ENERGY;
          default -> throw new IllegalStateException(STR."Unexpected value: \{c}");
        };
      }

      @Override
      public String toString() {
        return switch (this) {
          case EMPTY -> ".";
          case MIRROR_RIGHT -> "/";
          case MIRROR_LEFT -> "\\";
          case SPLITTER_HORIZ -> "-";
          case SPLITTER_VERT -> "|";
          case ENERGY -> "#";
        };
      }
    }

  }
}
