package day03;

import lib.Grid;
import lib.Input;
import lib.Pos;
import org.apache.commons.lang3.time.StopWatch;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.IntStream;

public class Solution {

  public record Schematic(Character[][] elements, List<Part> numbers, List<Pos> maybeGears,
                          boolean[][] symbols) implements Grid<Character> {

    static Schematic parseSchematic(List<String> lines) {
      int width = lines.size();
      int height = lines.getFirst().length();
      List<Part> parts_ = new ArrayList<>();
      List<Pos> maybeGears_ = new ArrayList<>();
      Character[][] elements_ = new Character[height][width];
      boolean[][] symbols_ = new boolean[height][width];

      for (int y = 0; y < lines.size(); y++) {
        final char[] row = lines.get(y).toCharArray();
        int numberStart = -1;
        StringBuilder digitStream = new StringBuilder();
        for (int x = 0; x < row.length; x++) {
          // Copy the character from the schema
          char c = row[x];
          boolean isDigit = Character.isDigit(c);
          elements_[y][x] = c;

          // Save where symbols are valid
          // TODO: Consider rewriting as a single-pass convolution
          if (c != '.' && !isDigit) {
            int up = Math.max(y - 1, 0);
            int down = Math.min(y + 1, height - 1);
            int left = Math.max(x - 1, 0);
            int right = Math.min(x + 1, width - 1);
            symbols_[up][left] = true;
            symbols_[up][x] = true;
            symbols_[up][right] = true;
            symbols_[y][left] = true;
            symbols_[y][x] = true;
            symbols_[y][right] = true;
            symbols_[down][left] = true;
            symbols_[down][x] = true;
            symbols_[down][right] = true;
          }

          if (c == '*') {
            maybeGears_.add(new Pos(x, y));
          }

          if (isDigit) {
            if (numberStart == -1) {
              numberStart = x;
            }
            digitStream.append(c);
          } else if (!digitStream.isEmpty()) {
            Pos start = new Pos(numberStart, y);
            Pos end = new Pos(x - 1, y);
            parts_.add(new Part(Integer.parseInt(digitStream.toString()), start, end));
            // Reset sentinel and stream
            numberStart = -1;
            digitStream = new StringBuilder();
          }
        }
        // Reached end of row and have a number
        if (!digitStream.isEmpty()) {
          Pos start = new Pos(numberStart, y);
          Pos end = new Pos(row.length - 1, y);
          parts_.add(new Part(Integer.parseInt(digitStream.toString()), start, end));
        }
      }

      return new Schematic(elements_, parts_, maybeGears_, symbols_);
    }

    public record Part(int value, Pos start, Pos end) {

    }

    public record Gear(Part a, Part b) {

      int ratio() {
        return a.value * b.value;
      }
    }

    boolean isPart(Part number) {
      int y = number.start.y();
      return IntStream.range(number.start.x(), number.end.x() + 1).anyMatch(x -> symbols[y][x]);
    }


    List<Gear> gears(List<Part> parts) {
      // Part numbers are indexed from 1 onwards
      int[][] partMap = new int[height()][width()];

      for (int partNum = 1; partNum <= parts.size(); partNum++) {
        Part part = parts.get(partNum - 1);
        int finalPartNum = partNum;
        IntStream.range(part.start.y(), part.end.y() + 1).forEach(
            y -> IntStream.range(part.start.x(), part.end.x() + 1)
                .forEach(x -> partMap[y][x] = finalPartNum));
      }

      return maybeGears.stream().map(maybe -> {
        List<Integer> adjacentParts = surroundingPositions(maybe).map(
            pos -> partMap[pos.y()][pos.x()]).filter(partNum -> partNum != 0).distinct().toList();
        if (adjacentParts.size() == 2) {
          return new Gear(parts.get(adjacentParts.get(0) - 1), parts.get(adjacentParts.get(1) - 1));
        }
        return null;
      }).filter(Objects::nonNull).toList();
    }

    public static void main(String[] args) throws IOException {
      StopWatch watch = new StopWatch();
      watch.start();
      Schematic schematic = Schematic.parseSchematic(Input.lines("day03/input.txt").toList());

      System.out.println(watch.getTime());
      List<Part> parts = schematic.numbers().stream().filter(schematic::isPart).toList();
      System.out.println(watch.getTime());

      int partOne = parts.stream().mapToInt(Part::value).sum();
      System.out.println(watch.getTime());

      int partTwo = schematic.gears(parts).stream().mapToInt(Gear::ratio).sum();
      System.out.println(watch.getTime());

      System.out.println(STR. "Part 1: \{ partOne }" );
      System.out.println(STR. "Part 2: \{ partTwo }" );
    }
  }
}
