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
      return new Pattern(block.lines()
          .map(line -> line.chars().mapToObj(c -> (char) c).toList())
          .toList());
    }

    public Pattern rotate() {
      return new Pattern(rotate(RotationDirection.CLOCKWISE));
    }

    Pair<Long, Long> getReflectionNote(int smudges) {
      var horizontal = findHorizontalReflectionIndex(smudges);
      if (horizontal != -1) {
        return new ImmutablePair<>(horizontal, -1L);
      }

      var vertical = findVerticalReflectionIndex(smudges);
      if (vertical == -1) {
        throw new IllegalStateException("No mirror found");
      }

      return new ImmutablePair<>(-1L, vertical);
    }

    long findVerticalReflectionIndex(int smudges) {
      return rotate().findHorizontalReflectionIndex(smudges);
    }

    long findHorizontalReflectionIndex(int smudges) {
      var image = elements();
      for (int y = 0; y < image.size() - 1; y++) {
        var rowA = image.get(y);
        var rowB = image.get(y + 1);

        var foundErrors = 0;
        for (int i = 0; i < width() && foundErrors <= smudges; i++) {
          if (!rowA.get(i).equals(rowB.get(i))) {
            foundErrors++;
          }
        }

        if (foundErrors <= 1) {
          int walkA = y - 1;
          int walkB = y + 2;
          while (walkA >= 0 && walkB < image.size()) {
            var walkRowA = image.get(walkA--);
            var walkRowB = image.get(walkB++);
            for (int i = 0; i < width() && foundErrors <= 1; i++) {
              if (!walkRowA.get(i).equals(walkRowB.get(i))) {
                foundErrors++;
              }
            }
          }
          if (foundErrors == smudges) {
            return y;
          }
        }
      }

      return -1;
    }
  }


  public static void main(String[] args) throws IOException {
    List<Pattern> patterns =
        Arrays.stream(Input.read("day13/input.txt").split("\n\n")).map(Pattern::fromLines).toList();

    long partOne = patterns.stream().mapToLong(pattern -> {
      Pair<Long, Long> result = pattern.getReflectionNote(0);
      if (result.getLeft() != -1) {
        return (result.getLeft() + 1) * 100;
      }
      return result.getRight() + 1;
    }).sum();

    long partTwo = patterns.stream().mapToLong(pattern -> {
      var result = pattern.getReflectionNote(1);
      if (result.getLeft() != -1) {
        return (result.getLeft() + 1) * 100;
      }
      return result.getRight() + 1;
    }).sum();

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }
}
