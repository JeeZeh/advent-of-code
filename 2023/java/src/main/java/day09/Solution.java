package day09;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import lib.Input;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<Sequence> sequences = Input.lines("day09/input.txt").map(Sequence::fromLine).toList();
//    System.out.println(Arrays.toString(sequences.getFirst().elements()));
//    System.out.println(sequences.get(0).extrapolate());;
    long partOne = sequences.stream().mapToLong(seq -> {
      var ext = seq.extrapolate();
      System.out.println(STR. "\{ seq.elements[seq.elements.length - 1] } => \{ ext }" );
      return ext;
    }).sum();

    System.out.println(STR. "Part 1: \{ partOne }" );
  }

  public record Sequence(long[] elements) {

    static Sequence fromLine(String line) {
      return new Sequence(Arrays.stream(line.split(" ")).mapToLong(Long::parseLong).toArray());
    }

    public long extrapolate() {
      var steps = this.getSteps();
//      System.out.println(Arrays.toString(steps.elements));
      if (steps.elements[steps.elements.length - 1] == 0) {
        return this.elements[elements.length - 1];
      }

      return elements[elements.length - 1] + steps.extrapolate();
    }

    public Sequence getSteps() {
      long[] steps = new long[elements.length - 1];
      for (int i = 0; i < elements.length - 1; i++) {
        steps[i] = elements[i + 1] - elements[i];
      }

      return new Sequence(steps);
    }
  }


}
