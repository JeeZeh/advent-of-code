package day09;

import com.google.common.primitives.Longs;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import lib.Input;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<Sequence> sequences = Input.lines("day09/input.txt").map(Sequence::fromLine).toList();

    List<Sequence> reversedSequences = sequences.stream().map(sequence -> {
      long[] copy = Longs.asList(sequence.elements).reversed().stream().mapToLong(l -> l).toArray();
      return new Sequence(copy, false);
    }).toList();

    long partOne = totalExtrapolations(sequences);
    long partTwo = totalExtrapolations(reversedSequences);

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  static long totalExtrapolations(List<Sequence> sequences) {
    return sequences.stream().mapToLong(Sequence::extrapolate).sum();
  }

  public record Sequence(long[] elements, boolean allZero) {

    static Sequence fromLine(String line) {
      return new Sequence(Arrays.stream(line.split(" ")).mapToLong(Long::parseLong).toArray(),
          false);
    }

    public long extrapolate() {
      var steps = this.getSteps();
      if (steps.allZero) {
        return this.elements[elements.length - 1];
      }

      return elements[elements.length - 1] + steps.extrapolate();
    }

    public Sequence getSteps() {
      long[] steps = new long[elements.length - 1];
      boolean allZero = true;
      for (int i = 0; i < elements.length - 1; i++) {
        steps[i] = elements[i + 1] - elements[i];
        if (steps[i] != 0) {
          allZero = false;
        }
      }

      return new Sequence(steps, allZero);
    }
  }
}
