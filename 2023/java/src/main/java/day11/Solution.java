package day11;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;
import lib.Grid;
import lib.Input;
import lib.Pos;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day11/input.txt").toList();
    var space = Space.fromLines(lines);
    long partOne = 0;
    long partTwo = 0;
    for (int a = 0; a < space.satellites().size() - 1; a++) {
      for (int b = a + 1; b < space.satellites().size(); b++) {
        var satA = space.satellites().get(a);
        var satB = space.satellites().get(b);
        partOne += space.distance(satA, satB, 2);
        partTwo += space.distance(satA, satB, 1_000_000);
      }
    }

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public record Space(Boolean[][] elements, List<Pos> satellites, boolean[] satelliteCols,
                      boolean[] satelliteRows) implements Grid<Boolean> {

    public long distance(Pos satA, Pos satB, long scale) {
      long gaps = 0;
      gaps += IntStream.range(Math.min(satA.x(), satB.x()), Math.max(satA.x(), satB.x()))
          .filter(col -> !this.satelliteCols[col]).count();
      gaps += IntStream.range(Math.min(satA.y(), satB.y()), Math.max(satA.y(), satB.y()))
          .filter(row -> !this.satelliteRows[row]).count();

      return satA.dist(satB) + (gaps * (scale - 1));
    }

    static Space fromLines(List<String> lines) {
      int height = lines.size();
      int width = lines.getFirst().length();
      Boolean[][] elements = new Boolean[height][width];
      boolean[] satelliteCols = new boolean[width];
      boolean[] satelliteRows = new boolean[height];
      List<Pos> satellites = new ArrayList<>();

      for (int y = 0; y < lines.size(); y++) {
        String row = lines.get(y);
        for (int x = 0; x < row.length(); x++) {
          elements[y][x] = row.charAt(x) == '#';
          if (elements[y][x]) {
            satelliteCols[x] = true;
            satelliteRows[y] = true;
            satellites.add(new Pos(x, y));
          }
        }
      }

      return new Space(elements, satellites, satelliteCols, satelliteRows);
    }
  }
}