package day18;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;
import java.util.stream.IntStream;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  public static void main(String[] args) throws IOException {
    var lines = Input.lines("day18/example.txt").toList();
    long partOne = shoeLace(
        getVertices(lines, (parts) -> Integer.parseInt(parts[1]), parts -> letterToDir(parts[0])));
    long partTwo = shoeLace(getVertices(lines,
        (parts) -> Integer.parseInt(parts[2].substring(2, parts[2].length() - 2), 16),
        parts -> numToDir(String.valueOf(parts[2].charAt(7)))));

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public static long shoeLace(List<Pos> verts) {
    long leftLace = IntStream.range(0, verts.size())
        .mapToLong(i -> (long) verts.get(i).x() * (long) verts.get((i + 1) % verts.size()).y())
        .sum();
    long rightLace = IntStream.range(0, verts.size())
        .mapToLong(i -> (long) verts.get(i).y() * (long) verts.get((i + 1) % verts.size()).x())
        .sum();

    // Pick's Theorem
    long perimeterArea = IntStream.range(0, verts.size())
        .mapToLong(i -> verts.get(i).dist(verts.get((i + 1) % verts.size()))).sum();

    return (Math.abs(leftLace - rightLace) / 2) + (perimeterArea / 2) + 1;

  }

  public static Direction letterToDir(String letter) {
    return switch (letter) {
      case "R" -> Direction.RIGHT;
      case "U" -> Direction.UP;
      case "D" -> Direction.DOWN;
      case "L" -> Direction.LEFT;
      default -> throw new IllegalStateException(STR."Unexpected letter: \{letter}");
    };
  }

  public static Direction numToDir(String num) {
    return switch (num) {
      case "0" -> Direction.RIGHT;
      case "1" -> Direction.DOWN;
      case "2" -> Direction.LEFT;
      case "3" -> Direction.UP;
      default -> throw new IllegalStateException(STR."Unexpected num: \{num}");
    };
  }

  public static List<Pos> getVertices(List<String> digPlan,
      Function<String[], Integer> parseDistance, Function<String[], Direction> parseDirection) {
    List<Pos> verts = new ArrayList<>();

    Pos walk = new Pos(0, 0);
    verts.add(walk);
    for (String row : digPlan) {
      var parts = row.split(" ");
      var dist = parseDistance.apply(parts);
      var direction = parseDirection.apply(parts);

      walk = direction.transpose(walk, dist);
      verts.add(walk);
    }

    return verts;
  }
}
