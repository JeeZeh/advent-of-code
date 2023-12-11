package day11;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Optional;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.function.BiFunction;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  static final long SPACE_SCALE = 1_000_000;


  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day11/input.txt").toList();
    var space = Space.fromLines(lines);
    Map<Pair, Long> pairs = new HashMap<>();
    // TODO: Don't use BFS for this, just:
    // 1. Walk between each pair in Manhattan style
    // 2. Count the 'gaps' encountered
    // 3. Add the distance + (gaps * scalar)
    space.satellites.stream().map(space::shortestPaths).forEach(pairs::putAll);
    long partOne = pairs.values().stream().mapToLong(i -> i).sum();
    System.out.println(STR."Part 1: \{partOne}");
//    System.out.println(STR."Part 2: \{enclosed.size()}");
  }


  public record Pair(Pos a, Pos b) {

  }

  public record Space(Boolean[][] elements, List<Pos> satellites, boolean[] satelliteCols,
                      boolean[] satelliteRows) implements Grid<Boolean> {

    public record Step(Pos pos, long distance) implements Comparable<Step> {

      @Override
      public int compareTo(Step o) {
        // Reverse comparison for PriorityQueue
        return Long.compare(this.distance, o.distance);
      }
    }

    public Map<Pair, Long> shortestPaths(Pos start) {
      final PriorityQueue<Step> queue = new PriorityQueue<>();
      final Set<Pos> seen = new HashSet<>();
      final Map<Pair, Long> pairDistances = new HashMap<>();
      seen.add(start);

      // Find next pipe locations
      nextSteps(start).forEach(queue::add);

      while (!queue.isEmpty()) {
        Step curr = queue.poll();

        if (this.elements[curr.pos.y()][curr.pos.x()]) {
          var sortedPair = Stream.of(start, curr.pos).sorted().toList();
          var pair = new Pair(sortedPair.getFirst(), sortedPair.getLast());
          pairDistances.computeIfAbsent(pair, (k) -> curr.distance);
        }

        if (!seen.contains(curr.pos)) {
          seen.add(curr.pos);
          nextSteps(curr.pos).filter(next -> !seen.contains(next.pos))
              .map(next -> new Step(next.pos, curr.distance + next.distance)).forEach(queue::add);
        }
      }

      return pairDistances;
    }

    Stream<Step> nextSteps(Pos a) {
      return this.surroundingPositions(a).map(b -> {
        long expanded = 1;
        if (!this.satelliteCols[b.x()]) {
          expanded += SPACE_SCALE - 1;
        }
        if (!this.satelliteRows[b.y()]) {
          expanded += SPACE_SCALE - 1;
        }
        return new Step(b, a.dist(b) * expanded);
      });
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