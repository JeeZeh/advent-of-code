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

    long partOne = garden.getReachable(garden.start, 64).getOrDefault(64, new HashSet<>()).size();
    long partTwo = 0;
    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public record Garden(List<List<Boolean>> elements, Pos start) implements Grid<Boolean> {

    public Map<Integer, Set<Pos>> getReachable(Pos start, long maxSteps) {
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
