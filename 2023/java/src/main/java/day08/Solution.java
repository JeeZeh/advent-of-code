package day08;

import java.io.IOException;
import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import lib.Input;
import lib.Number;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day08/input.txt").toList();
    List<Direction> directions = lines.get(0).chars()
        .mapToObj(c -> (char) c == 'L' ? Direction.LEFT : Direction.RIGHT).toList();

    var nodes = lines.stream().skip(2).map(Node::fromLine).toList();
    var nodeMap = nodes.stream().collect(Collectors.toMap(node -> node.id, node -> node));
    List<Node> ghosts = nodes.stream().filter(node -> node.id.charAt(2) == 'A').toList();

    System.out.println(STR. "Part 1: \{ partOne(nodeMap, directions, nodeMap.get("AAA")) }" );
    System.out.println(STR. "Part 2: \{ partTwo(nodeMap, directions, ghosts) }" );
  }

  static long partOne(Map<String, Node> nodes, List<Direction> directions, Node start) {
    Node location = start;
    final int dirCount = directions.size();
    int step = 0;
    while (location.id.charAt(2) != 'Z') {
      int stepIdx = step++ % dirCount;
      var dir = directions.get(stepIdx);
      location = nodes.get(location.move(dir));
    }

    return step;
  }

  static long partTwo(Map<String, Node> nodes, List<Direction> directions, List<Node> ghosts) {
    return ghosts.stream().map(ghost -> partOne(nodes, directions, ghost))
        .reduce((steps, lcm) -> Number.lcm(lcm, steps)).get();
  }

  public record Node(String id, String left, String right) {

    public String move(Direction direction) {
      return switch (direction) {
        case LEFT -> left;
        case RIGHT -> right;
      };
    }

    static Node fromLine(String line) {
      var parts = line.split(" = ");
      var id = parts[0];
      var edges = parts[1].substring(1, parts[1].length() - 1).split(", ");
      return new Node(id, edges[0], edges[1]);
    }
  }

  public enum Direction {
    LEFT, RIGHT
  }
}
