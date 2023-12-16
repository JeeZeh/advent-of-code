package day15;

import java.io.IOException;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.List;
import java.util.Objects;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.LongStream;
import lib.Input;

public class Solution {

  public record SequencePart(String part, long hash, String label, long labelHash, Operation op,
                             long value) {

    static SequencePart fromString(String s) {
      var hash = hash(s);
      var operation = s.endsWith("-") ? Operation.REMOVE : Operation.ADD;
      var parts = s.split(operation == Operation.ADD ? "=" : "-");
      var labelHash = hash(parts[0]);
      return new SequencePart(s, hash, parts[0], labelHash, operation,
          operation == Operation.ADD ? Long.parseLong(s.split("=")[1]) : 0L);
    }

    public static long hash(String part) {
      AtomicLong acc = new AtomicLong(0);
      part.chars().forEach((ascii) -> {
        acc.addAndGet(ascii);
        acc.set(acc.get() * 17);
        acc.set(acc.get() % 256);
      });

      return acc.get();
    }

    @Override
    public boolean equals(Object obj) {
      if (obj instanceof SequencePart) {
        return this.label.equals(((SequencePart) obj).label);
      }
      return false;
    }
  }

  public enum Operation {
    ADD, REMOVE;
  }

  public static void main(String[] args) throws IOException {
    List<SequencePart> parts = Arrays.stream(Input.read("day15/input.txt").split(","))
        .map(SequencePart::fromString).toList();

    long partOne = parts.stream().mapToLong(SequencePart::hash).sum();
    long partTwo = partTwo(parts);
    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public static long partTwo(List<SequencePart> parts) {
    LinkedList<SequencePart>[] boxes = new LinkedList[256];

    for (SequencePart part : parts) {
      var targetBox = boxes[(int) part.labelHash];
      if (targetBox == null) {
        targetBox = new LinkedList<>();
      }

      if (part.op == Operation.ADD) {
        var index = targetBox.indexOf(part);
        if (index == -1) {
          targetBox.add(part);
        } else {
          targetBox.set(index, part);
        }
      } else {
        targetBox.remove(part);
      }
      boxes[(int) part.labelHash] = targetBox;
    }

    return Arrays.stream(boxes).filter(Objects::nonNull)
        .mapToLong(box -> LongStream.range(0, box.size()).map(idx -> {
          var part = box.get((int) idx);
          return (1 + part.labelHash) * (idx + 1) * (part.value);
        }).sum()).sum();
  }
}
