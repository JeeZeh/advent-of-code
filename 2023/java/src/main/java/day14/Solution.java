package day14;

import java.io.IOException;
import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;
import org.checkerframework.checker.units.qual.A;
import lib.Grid;
import lib.Input;
import lib.Pos;

public class Solution {

  public record Dish(List<List<Tile>> elements) implements Grid<Tile> {
    public static Dish fromString(String chunk) {
      List<List<Tile>> lines = chunk.lines()
          .map(line -> line.chars().mapToObj(c -> Tile.fromChar((char) c)).toList())
          .toList();

      return new Dish(lines);
    }

    public long getLoad() {
      long load = 0;
      for (int i = 0; i < elements.size(); i++) {
        load +=
            elements.get(i).stream().filter(tile -> tile == Tile.ROUND).count() * (elements.size()
                - i);
      }

      return load;
    }


    public Dish cycle() {
      var image = this;
      image = image.makeFall(Pos.Direction.UP);
      image = image.makeFall(Pos.Direction.LEFT);
      image = image.makeFall(Pos.Direction.DOWN);
      image = image.makeFall(Pos.Direction.RIGHT);
      return image;
    }

    public Dish makeFall(Pos.Direction direction) {
      var clockwiseRotations = switch (direction) {
        case UP -> 1;
        case DOWN -> 3; // TODO: Simplify rotation to -1
        case LEFT -> 0;
        case RIGHT -> 2;
        default ->
            throw new IllegalStateException(STR. "Unsupported fall direction \{ direction }" );
      };

      var image = this;

      for (int i = 0; i < clockwiseRotations; i++) {
        image = new Dish(image.rotate(RotationDirection.COUNTER_CLOCKWISE));
      }

      var fellOff = image.elements().stream().map(row -> {
        List<Tile> outputRow = new ArrayList<>(row.size());

        List<Tile> split = new ArrayList<>();
        for (Tile el : row) {
          if (el == Tile.CUBE) {
            outputRow.addAll(split.stream().sorted().toList());
            outputRow.add(el);
            split = new ArrayList<>();
          } else {
            split.add(el);
          }
        }

        if (!split.isEmpty()) {
          outputRow.addAll(split.stream().sorted().toList());
        }
        return outputRow;
      }).toList();

      var adjustedImage = new Dish(fellOff);

      for (int i = 0; i < clockwiseRotations; i++) {
        adjustedImage = new Dish(adjustedImage.rotate(RotationDirection.CLOCKWISE));
      }

      return adjustedImage;
    }
  }


  public enum Tile {
    ROUND,
    EMPTY,
    CUBE;


    @Override
    public String toString() {
      return switch (this) {
        case ROUND -> "O";
        case EMPTY -> ".";
        case CUBE -> "#";
      };
    }

    public static Tile fromChar(char c) {
      return switch (c) {
        case 'O' -> ROUND;
        case '#' -> CUBE;
        case '.' -> EMPTY;
        default -> throw new IllegalStateException(STR. "Unexpected value: \{ c }" );
      };
    }
  }

  public static void main(String[] args) throws IOException {
    Dish dish = Dish.fromString(Input.read("day14/input.txt"));
    var wentNorth = dish.makeFall(Pos.Direction.UP);

    long partOne = wentNorth.getLoad();
    Map<String, Long> seen = new HashMap<>();

    long maxCycles = 1_000_000_000;
    boolean skipped = false;
    var cycled = dish;
    for (long cycle = 0; cycle < maxCycles; cycle++) {
      cycled = cycled.cycle();
      var fingerprint = cycled.asString();
      var lastSeen = seen.get(fingerprint);
      if (lastSeen != null && !skipped) {
        var gap = cycle - lastSeen;
        var multiples = (maxCycles - cycle) / gap;
        cycle += gap * multiples;
        skipped = true;
      }
      seen.put(cycled.asString(), cycle);
    }

    System.out.println(cycled.asString());

    long partTwo = cycled.getLoad();

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }
}
