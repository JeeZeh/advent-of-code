package day15;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import lib.Input;

public class Solution {

  public record SequencePart(String part) {

    public int hash(int initial) {
      AtomicInteger acc = new AtomicInteger(initial);
      part.chars().forEach((ascii) -> acc.set((acc.addAndGet(ascii) * 17) % 256));
      return acc.get();
    }
  }

  public static void main(String[] args) throws IOException {
    List<SequencePart> parts = Arrays.stream(Input.read("day15/input.txt").split(","))
        .map(SequencePart::new).toList();

    int partOne = parts.stream().mapToInt(part -> part.hash(0)).sum();
    System.out.println(STR."Part 1: \{partOne}");
  }
}
