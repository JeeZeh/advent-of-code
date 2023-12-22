package day22;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Comparator;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Collectors;
import lib.Input;

public class Solution {

  public record Point(int x, int y, int z) {

    Point move(int dx, int dy, int dz) {
      return new Point(x + dx, y + dy, z + dz);
    }

    public static Point fromString(String s) {
      var parts = Arrays.stream(s.split(",")).mapToInt(Integer::parseInt).toArray();

      return new Point(parts[0], parts[1], parts[2]);
    }

    @Override
    public String toString() {
      return STR."\{x},\{y},\{z}";
    }
  }

  public static class Brick {

    public void setId(String id) {
      this.id = id;
    }

    String id;
    final Point from;
    final Point to;

    final Set<Brick> above = new HashSet<>();
    final Set<Brick> below = new HashSet<>();

    public Brick(String id, Point from, Point to) {
      this.id = id;
      this.from = from;
      this.to = to;
    }

    public Brick(String id, Point from, Point to, Set<Brick> below, Set<Brick> above) {
      this.id = id;
      this.from = from;
      this.to = to;
      this.below.addAll(below);
      this.above.addAll(above);
    }

    public void addAbove(Brick brick) {
      this.above.add(brick);
    }

    public void addBelow(Brick brick) {
      this.below.add(brick);
    }

    public Brick move(int dx, int dy, int dz) {
      return new Brick(this.id, from.move(dx, dy, dz), to.move(dx, dy, dz), this.below, this.above);
    }

    public boolean restingOn(Brick other) {
      var above = this.from.z == other.to.z + 1;
      var xOverlap = this.from.x <= other.to.x && this.to.x >= other.from.x;
      var yOverlap = this.from.y <= other.to.y && this.to.y >= other.from.y;
      return above && xOverlap && yOverlap;
    }

    public Brick fall(Map<Integer, List<Brick>> settled, int highest) {
      int z = highest;
      while (z > 0) {
        if (settled.containsKey(z)) {
          var placeAbove = this.move(0, 0, -((this.from.z - z) - 1));
          var restingOn = settled.get(z).stream().filter(placeAbove::restingOn).toList();
          if (!restingOn.isEmpty()) {
            restingOn.forEach(below -> {
              below.addAbove(placeAbove);
              placeAbove.addBelow(below);
            });
            return placeAbove;
          }
        }
        z--;
      }
      return this.move(0, 0, -(this.from.z - 1));
    }

    public static Brick fromLine(String id, String line) {
      var parts = line.split("~");

      return new Brick(id, Point.fromString(parts[0]), Point.fromString(parts[1]));
    }

    public String getId() {
      return id;
    }

    @Override
    public String toString() {
      return STR."{ID=\{id}, POS=(\{from} -> \{to}), CONN=([\{below.stream().map(Brick::getId)
          .collect(Collectors.joining(", "))}], [\{above.stream().map(Brick::getId)
          .collect(Collectors.joining(", "))}])}";
    }
  }

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day22/input.txt").toList();
    List<Brick> bricks = new ArrayList<>();
    for (int i = 0; i < lines.size(); i++) {
      bricks.add(Brick.fromLine(STR."\{i}", lines.get(i)));
    }
    bricks.sort(Comparator.comparingInt(a -> a.from.z));
    for (int i = 0; i < bricks.size(); i++) {
      bricks.get(i).setId(STR."\{i}");
    }
    bricks = settle(bricks);

    Set<Brick> canDisintegrate = canDisintegrate(bricks);
    long partOne = canDisintegrate.size();
    System.out.println(STR."Part 1: \{partOne}");
    long partTwo = bricks.stream().mapToInt(Solution::disintegrate).sum();
    System.out.println(STR."Part 2: \{partTwo}");
  }

  static int disintegrate(Brick brick) {
    Deque<Brick> nextBricks = new ArrayDeque<>(brick.above);
    Set<Brick> disintegrated = new HashSet<>(List.of(brick));

    while (!nextBricks.isEmpty()) {
      var current = nextBricks.poll();

      if (disintegrated.containsAll(current.below)) {
        disintegrated.add(current);
        nextBricks.addAll(current.above);
      }
    }

    return disintegrated.size() - 1;
  }

  static Set<Brick> canDisintegrate(List<Brick> bricks) {
    Set<Brick> canDisintegrate = new HashSet<>(bricks);

    for (Brick brick : bricks) {
      var supportedBy = brick.below;
      // Don't consider any bricks which are found to be the sole-supporter of any other brick
      if (supportedBy.size() == 1) {
        canDisintegrate.removeAll(supportedBy);
      }
    }

    return canDisintegrate;
  }


  static List<Brick> settle(List<Brick> bricks) {
    Map<Integer, List<Brick>> settled = new HashMap<>();

    int highest = 0;
    for (Brick brick : bricks) {
      Brick fell = brick.fall(settled, highest);
      highest = Math.max(highest, fell.to.z);
      var list = settled.computeIfAbsent(fell.to.z, (k) -> new ArrayList<>());
      list.add(fell);
    }

    return settled.values().stream().flatMap(Collection::stream).toList();
  }
}
