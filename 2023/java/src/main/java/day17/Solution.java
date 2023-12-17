package day17;

import java.io.IOException;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Objects;
import java.util.PriorityQueue;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;
import org.apache.commons.lang3.time.StopWatch;

public class Solution {

  final static Direction[] directions = new Direction[]{Direction.UP, Direction.RIGHT,
      Direction.DOWN, Direction.LEFT};


  public static void main(String[] args) throws IOException {
    City city = City.fromLines(Input.lines("day17/input.txt").toList());
    StopWatch stopWatch = new StopWatch();
    stopWatch.start();
    System.out.println(STR."Part 1: \{city.bestHeatLoss(new Pos(0, 0), 1, 3)}");
    System.out.println(STR."Part 2: \{city.bestHeatLoss(new Pos(0, 0), 4, 10)}");
    System.out.println(stopWatch.getNanoTime());
    ;
  }


  record City(List<List<Long>> elements, Pos end) implements Grid<Long> {

    public record State(Pos pos, int heading, int steps) {

    }

    public record Crucible(Pos pos, int heading, int steps, long heatLoss) {

      State state() {
        return new State(pos, heading, steps);
      }
    }

    public long bestHeatLoss(Pos start, int minStep, int maxStep) {
      PriorityQueue<Crucible> nextSteps = new PriorityQueue<>(
          Comparator.comparingLong(a -> a.heatLoss));
      Map<State, Long> heatLosses = new HashMap<>();

      nextSteps.add(new Crucible(start, 1, 0, 0));
      nextSteps.add(new Crucible(start, 2, 0, 0));

      while (!nextSteps.isEmpty()) {
        var current = nextSteps.poll();

        // Get possible options from here
        // Add to next steps
        getOptions(current, minStep, maxStep).forEach(next -> {
          var state = next.state();
          if (next.heatLoss < heatLosses.computeIfAbsent(state, k -> Long.MAX_VALUE)) {
            heatLosses.put(state, next.heatLoss);
            nextSteps.add(next);
          }
        });

        // Prioritize by minimizing potential heat loss from that point to the end
        // Repeat until we reach the end, update the max heat loss to the end
        // Don't consider new options if it exceeds the max heat loss seen previously (real, not potential)

        // I think we'll need back tracking and memoization? How do we know that when we arrive,
        // that we actually took the optimal path?
      }

      return heatLosses.entrySet().stream().filter(e -> e.getKey().pos.equals(end))
          .mapToLong(Entry::getValue).min().getAsLong();
    }

    Stream<Crucible> getOptions(Crucible current, int minStep, int maxStep) {
      var left = turn(current, -1);
      var right = turn(current, 1);

      return Stream.concat(
              IntStream.range(minStep, maxStep + 1).mapToObj(times -> step(left, times)),
              IntStream.range(minStep, maxStep + 1).mapToObj(times -> step(right, times)))
          .filter(Objects::nonNull);
    }

    Crucible turn(Crucible current, int delta) {
      int newDirection = (4 + current.heading + delta) % 4;
      return new Crucible(current.pos, newDirection, 0, current.heatLoss);
    }

    Crucible step(Crucible current, int steps) {
      Pos newPos = current.pos;
      long heatLoss = current.heatLoss;
      for (int i = 0; i < steps; i++) {
        newPos = directions[current.heading].transpose(newPos);
        if (!isWithin(newPos)) {
          return null;
        }
        heatLoss += get(newPos);
      }
      return new Crucible(newPos, current.heading, steps, heatLoss);
    }

    static City fromLines(List<String> lines) {
      return new City(lines.stream().map(
              line -> line.chars().mapToObj(c -> Long.parseLong(String.valueOf((char) c))).toList())
          .toList(), new Pos(lines.getFirst().length() - 1, lines.size() - 1));
    }
  }
}
