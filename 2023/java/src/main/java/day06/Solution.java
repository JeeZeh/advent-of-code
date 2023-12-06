package day06;

import static java.lang.StringTemplate.STR;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.LongStream;

import com.google.common.collect.Lists;
import lib.Input;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day06/input.txt").toList();

    List<List<String>> splits = lines.stream().map(line -> Arrays.stream(line.split("\\s+")).skip(1).toList()).toList();

    StringBuilder superTime = new StringBuilder();
    StringBuilder superDistance = new StringBuilder();

    List<Race> races = new ArrayList<>();
    for (int i = 0; i < splits.get(0).size(); i++) {
      String time = splits.get(0).get(i);
      String dist = splits.get(1).get(i);
      superTime.append(time);
      superDistance.append(dist);
      races.add(new Race(Long.parseLong(time), Long.parseLong(dist)));
    }
    Race superRace = new Race(Long.parseLong(superTime.toString()), Long.parseLong(superDistance.toString()));

    long partOne = races.stream().mapToLong(race -> race.getStrategies().stream().filter(strat -> strat.beats).count()).reduce(1, (x, y) -> x * y);
    long partTwo = superRace.getStrategies().stream().filter(strat -> strat.beats).count();

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }


  public record Race(long time, long record) {

    public record Strategy(long holdTime, boolean beats) {
    }

    long getDistance(long holdTime) {
      return (time - holdTime) * holdTime;
    }

    boolean beats(long result) {
      return result > record;
    }

    List<Strategy> getStrategies() {
      return LongStream.range(0, time).mapToObj(hold -> new Strategy(hold, beats(getDistance(hold)))).toList();
    }
  }


}
