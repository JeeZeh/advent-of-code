package day17;

import java.io.IOException;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Set;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  final static Direction[] directions = new Direction[]{Direction.UP, Direction.RIGHT,
      Direction.DOWN, Direction.LEFT};


  public static void main(String[] args) throws IOException {
    City city = City.fromLines(Input.lines("day16/input.txt").toList());
  }


  record City(List<List<Long>> elements) implements Grid<Long> {

    public record Crucible(Pos pos, int heading, long potentialLoss, int straightStep) implements
        Comparable<Crucible> {

      @Override
      public int compareTo(Crucible o) {
        return Long.compare(this.potentialLoss, o.potentialLoss);
      }
    }

    public Crucible findHeatLoss(Pos start) {
      PriorityQueue<Crucible> nextSteps = new PriorityQueue<>();
      Set<Crucible> seen = new HashSet<>();

      while (!nextSteps.isEmpty()) {
        var curr = nextSteps.poll();

        if (seen.contains(curr)) {
          continue;
        } else {
          seen.add(curr);
        }

        // Get possible options from here
        // Add to next steps
        // Prioritize by minimizing potential heat loss from that point to the end
        // Repeat until we reach the end, update the max heat loss to the end
        // Don't consider new options if it exceeds the max heat loss seen previously (real, not potential)

        // I think we'll need back tracking and memoization? How do we know that when we arrive,
        // that we actually took the optimal path?
      }

      return seen.stream().filter(c -> c.pos.x() == width() - 1 && c.pos.y() == height() - 1)
          .findFirst().get();
    }

    Stream<Crucible> getOptions(Crucible current) {
      var leftDirection = directions[(current.heading - 1) % 4];
      var rightDirection = directions[(current.heading + 1) % 4];

      // TODO
      return Stream.of(current);
    }

    static City fromLines(List<String> lines) {
      return new City(lines.stream().map(
              line -> line.chars().mapToObj(c -> Long.parseLong(String.valueOf((char) c))).toList())
          .toList());
    }
  }
}
