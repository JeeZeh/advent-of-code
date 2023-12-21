package day21;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import lib.Grid;
import lib.Input;
import lib.Pos;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day21/input.txt").toList();
    final Garden garden = Garden.fromLines(lines);

    long partOne = garden.getReachable(64).getOrDefault(64, new HashSet<>()).size();
    long partTwo = getPartTwo(garden, 26501365);

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  private static long getPartTwo(Garden garden, int goal) {
    // https://www.reddit.com/r/adventofcode/comments/18nevo3/2023_day_21_solutions/keaiiq7/?context=3
    var maxSteps = garden.start.x() + (garden.width() * 2L);
    Garden expanded = garden.expand();
    Map<Integer, Set<Pos>> maxSpaces = expanded.getReachable(maxSteps);

    var a0 = maxSpaces.get(garden.start.x()).size();
    var a1 = maxSpaces.get(garden.start.x() + (garden.width())).size();
    var a2 = maxSpaces.get(garden.start.x() + garden.width() * 2).size();
    var n = goal / garden.width();

    var b0 = a0;
    var b1 = a1 - a0;
    var b2 = a2 - a1;
    return b0 + (long) b1 * n + ((long) n * (n - 1) / 2) * (b2 - b1);
  }

  public record Garden(List<List<Boolean>> elements, Pos start) implements Grid<Boolean> {

    public Garden expand() {
      List<List<Boolean>> rows = new ArrayList<>();

      for (int i = 0; i < 5; i++) {
        for (int y = 0; y < height(); y++) {
          List<Boolean> row = new ArrayList<>();
          for (int i1 = 0; i1 < 5; i1++) {
            row.addAll(this.elements.get(y));
          }
          rows.add(row);
        }
      }
      return new Garden(rows, new Pos(rows.size() / 2, rows.getFirst().size() / 2));
    }

    public Map<Integer, Set<Pos>> getReachable(long maxSteps) {
      Deque<Pair<Pos, Integer>> nextSteps = new ArrayDeque<>();
      Map<Integer, Set<Pos>> reachable = new HashMap<>();

      nextSteps.add(new ImmutablePair<>(start, 0));
      reachable.put(0, new HashSet<>(List.of(start)));

      while (!nextSteps.isEmpty()) {
        var current = nextSteps.poll();

        surroundingPositionsCardinal(current.getLeft()).map(
                newPos -> new ImmutablePair<>(newPos, current.getRight() + 1))
            .filter(next -> !get(next.getLeft())).filter(next -> next.getRight() <= maxSteps)
            .filter(next -> !reachable.computeIfAbsent(next.getRight(), k -> new HashSet<>())
                .contains(next.getLeft())).forEach(next -> {
              reachable.get(next.getRight()).add(next.getLeft());
              nextSteps.add(next);
            });
      }

      return reachable;
    }


    public static Garden fromLines(List<String> lines) {
      Pos start = null;
      int height = lines.size();
      int width = lines.getFirst().length();
      List<List<Boolean>> rows = new ArrayList<>(height);

      for (int y = 0; y < height; y++) {
        List<Boolean> row = new ArrayList<>(width);
        for (int x = 0; x < width; x++) {
          var c = lines.get(y).charAt(x);
          if (c == 'S') {
            start = new Pos(x, y);
          }
          row.add(c == '#');
        }
        rows.add(row);
      }

      assert start != null;
      return new Garden(rows, start);
    }
  }
}
