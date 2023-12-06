package day06;

import static java.lang.StringTemplate.STR;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.IntStream;
import java.util.stream.LongStream;
import com.google.common.collect.Lists;
import lib.Input;

public class Solution {
  List<String> lines = Input.lines("day06/example.txt").toList();


  public record Race(int time, int record) {

    public record Strategy(int holdTime, boolean beats) {
    }

    int getDistance(int holdTime) {
      return (time - holdTime) * holdTime;
    }

    boolean beats(int result) {
      return result > record;
    }

    List<Strategy> getStrategies() {
      return IntStream.range(0, time)
          .mapToObj(hold -> new Strategy(hold, beats(getDistance(hold))))
          .toList();
    }
  }

  public Solution() throws IOException {
    List<List<String>> splits =
        lines.stream().map(line -> Arrays.stream(line.split("\\s+")).skip(1).toList()).toList();

    List<Race> races = new ArrayList<>();
    for (int i = 0; i < splits.get(0).size(); i++) {
      races.add(new Race(
          Integer.parseInt(splits.get(0).get(i)),
          Integer.parseInt(splits.get(1).get(i))));
    }

    int partOne = races.stream()
        .mapToInt(race -> (int) race.getStrategies().stream().filter(strat -> strat.beats).count())
        .reduce(1, (x, y) -> x * y);
  }
}
