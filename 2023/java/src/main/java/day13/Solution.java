package day13;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.IntStream;
import lib.Grid;
import lib.Input;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;

public class Solution {

  public record Pattern(List<List<Character>> elements) implements Grid<Character> {

    static Pattern fromLines(String block) {
      return new Pattern(
          block.lines().map(line -> line.chars().mapToObj(c -> (char) c).toList()).toList());
    }

    public Pattern rotate() {
      return new Pattern(rotate(RotationDirection.CLOCKWISE));
    }

    Pair<Long, Long> getReflectionNote(Pair<Long, Long> avoidMatch) {
      var horizontal = findHorizontalReflectionIndex(avoidMatch == null ? 0 : avoidMatch.getRight());
      if (horizontal != -1) {
        return new ImmutablePair<>(horizontal, 0L);
      }

      var vertical = findVerticalReflectionIndex(avoidMatch == null ? 0 : avoidMatch.getLeft());
      if (vertical == -1) {
        throw new IllegalStateException("No mirror found");
      }

      return new ImmutablePair<>(0L, vertical);
    }

    long findVerticalReflectionIndex(long skipLine) {
      return rotate().findHorizontalReflectionIndex(skipLine);
    }

    long findHorizontalReflectionIndex(long skipLine) {
      var image = elements();
      for (int y = 0; y < image.size() - 1; y++) {
        var rowA = image.get(y);
        var rowB = image.get(y + 1);

        var allowedErrors = skipLine > 0L ? 1 : 0;
        if (rowA.equals(rowB)) {
          int walkA = y - 1;
          int walkB = y + 2;
          while (walkA >= 0 && walkB < image.size()) {
            var walkRowA = image.get(walkA--);
            var walkRowB = image.get(walkB++);
            for (int i = 0; i < width() && allowedErrors >= 0; i++) {
              if (!walkRowA.get(i).equals(walkRowB.get(i))) {
                allowedErrors--;
              }
            }
          }
          if (allowedErrors >= 0 && (skipLine == 0 || skipLine != y)) {
            return y;
          }
        }
      }

      return -1;
    }
  }


  public static void main(String[] args) throws IOException {
    List<Pattern> patterns = Arrays.stream(Input.read("day13/input.txt").split("\n\n"))
        .map(Pattern::fromLines).toList();
    List<Pair<Long, Long>> seenReflections = new ArrayList<>();

    long partOne = patterns.stream().mapToLong(pattern -> {
      System.out.println(pattern.asString());
      Pair<Long, Long> result = pattern.getReflectionNote(null);
      System.out.println(result);
      seenReflections.add(result);
      if (result.getLeft() != 0) {
        return (result.getLeft() + 1) * 100;
      }
      return result.getRight() + 1;
    }).sum();

    long partTwo = IntStream.range(0, patterns.size()).mapToLong(i -> {
      var result = patterns.get(i).getReflectionNote(seenReflections.get(i));
      if (result.getLeft() != 0) {
        return (result.getLeft() + 1) * 100;
      }
      return result.getRight() + 1;
    }).sum();

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }
}
