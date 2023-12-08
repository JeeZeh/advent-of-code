package day02;

import java.io.IOException;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import lib.Input;

public class Solution {

  public record Set(int red, int green, int blue) {

    boolean withinBounds(Set other) {
      return this.red <= other.red && this.green <= other.green && this.blue <= other.blue;
    }
  }


  public record Game(int id, List<Set> sets) {

    boolean canPlay(Set limit) {
      return sets.stream().allMatch(set -> set.withinBounds(limit));
    }
  }

  public static void main(String[] args) throws IOException {
    List<Game> games = Input.lines("day02/input.txt").map(Solution::parseGame).toList();

    Set partOneLimit = new Set(12, 13, 14);
    int partOne = games.stream().filter(game -> game.canPlay(partOneLimit)).map(Game::id)
        .mapToInt(Integer::intValue).sum();

    int partTwo = games.stream().map(game -> {
      AtomicInteger minRed = new AtomicInteger();
      AtomicInteger minGreen = new AtomicInteger();
      AtomicInteger minBlue = new AtomicInteger();

      game.sets.forEach(set -> {
        if (set.red > minRed.get()) {
          minRed.set(set.red);
        }
        if (set.green > minGreen.get()) {
          minGreen.set(set.green);
        }
        if (set.blue > minBlue.get()) {
          minBlue.set(set.blue);
        }
      });

      return minRed.get() * minGreen.get() * minBlue.get();
    }).mapToInt(Integer::intValue).sum();
    System.out.println(partOne);
    System.out.println(partTwo);
  }

  static Set parseSet(String set) {
    AtomicInteger red = new AtomicInteger();
    AtomicInteger green = new AtomicInteger();
    AtomicInteger blue = new AtomicInteger();

    Arrays.stream(set.split(", ")).map(color -> color.split(" ")).forEach(parts -> {
      int count = Integer.parseInt(parts[0]);
      switch (parts[1]) {
        case "red" -> red.set(count);
        case "green" -> green.set(count);
        case "blue" -> blue.set(count);
      }
    });

    return new Set(red.get(), green.get(), blue.get());
  }

  static Game parseGame(String line) {
    String[] parts = line.split(": ");
    int gameId = Integer.parseInt(parts[0].split(" ")[1]);

    List<Set> sets = Arrays.stream(parts[1].split("; ")).map(Solution::parseSet).toList();
    return new Game(gameId, sets);
  }
}
