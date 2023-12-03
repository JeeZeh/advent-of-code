package day03;

import lib.Grid;
import lib.Input;
import lib.Pos;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;

public class Solution {

  public record Schematic(Character[][] elements) implements Grid<Character> {
    static Schematic parseSchematic(List<String> lines) {
      return new Schematic(lines.stream().map(line -> line.chars().mapToObj(c -> (char) c).toArray(Character[]::new)).toArray(Character[][]::new));
    }

    public record Part(int value, Pos start, Pos end) {
    }

    public record Gear(Part a, Part b) {
      int ratio() {
        return a.value * b.value;
      }
    }

    boolean isPart(Part number) {
      return surroundingPositions(number.start, number.end).anyMatch(pos -> {
        char check = elements[pos.y()][pos.x()];
        return check != '.' && !Character.isDigit(check);
      });
    }


    List<Gear> gears(List<Part> parts) {
      // Part numbers are indexed from 1 onwards
      int[][] partMap = new int[height()][width()];

      for (int partNum = 1; partNum <= parts.size(); partNum++) {
        Part part = parts.get(partNum - 1);
        int finalPartNum = partNum;
        IntStream.range(part.start.y(), part.end.y() + 1).forEach(y -> IntStream.range(part.start.x(), part.end.x() + 1).forEach(x -> partMap[y][x] = finalPartNum));
      }

      List<Gear> gears = new ArrayList<>();
      for (int row = 0; row < height(); row++) {
        for (int col = 0; col < width(); col++) {
          if (elements[row][col] == '*') {
            List<Integer> adjacentParts = surroundingPositions(new Pos(col, row)).map(pos -> partMap[pos.y()][pos.x()]).filter(partNum -> partNum != 0).distinct().toList();
            if (adjacentParts.size() == 2) {
              gears.add(new Gear(parts.get(adjacentParts.get(0) - 1), parts.get(adjacentParts.get(1) - 1)));
            }
          }
        }
      }
      return gears;
    }


    List<Part> numbers() {
      List<Part> numbers = new ArrayList<>();

      for (int row = 0; row < height(); row++) {
        int numberStart = -1;
        StringBuilder digitStream = new StringBuilder();
        for (int col = 0; col < width(); col++) {
          char c = elements[row][col];
          if (Character.isDigit(c)) {
            if (numberStart == -1) {
              numberStart = col;
            }
            digitStream.append(c);
          } else if (!digitStream.isEmpty()) {
            Pos start = new Pos(numberStart, row);
            Pos end = new Pos(col - 1, row);
            numbers.add(new Part(Integer.parseInt(digitStream.toString()), start, end));
            // Reset sentinel and stream
            numberStart = -1;
            digitStream = new StringBuilder();
          }
        }
        // Reached end of row and have a number
        if (!digitStream.isEmpty()) {
          Pos start = new Pos(numberStart, row);
          Pos end = new Pos(width() - 1, row);
          numbers.add(new Part(Integer.parseInt(digitStream.toString()), start, end));
        }
      }
      return numbers;
    }


    public static void main(String[] args) throws IOException {
      Schematic schematic = Schematic.parseSchematic(Input.lines("day03/input.txt").toList());
      List<Part> parts = schematic.numbers().stream().filter(schematic::isPart).toList();

      int partOne = parts.stream().mapToInt(Part::value).sum();
      int partTwo = schematic.gears(parts).stream().mapToInt(Gear::ratio).sum();
      System.out.println(STR. "Part 1: \{ partOne }" );
      System.out.println(STR. "Part 2: \{ partTwo }" );
    }
  }
}
