package day05;

import com.google.common.collect.Lists;
import lib.Input;

import java.io.IOException;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class Solution {

  public record AMap(long dest, long source, long length) {

    long apply(long number) {
      if (number > source + length - 1 || number < source) {
        return number;
      }

      return (number - source) + dest;
    }

    static AMap fromLine(String line) {
      List<Long> nums = Arrays.stream(line.split(" ")).map(Long::parseLong).toList();
      return new AMap(nums.get(0), nums.get(1), nums.get(2));
    }
  }

  public record Almanac(String from, String to, List<AMap> maps) {
    long map(long number) {
      for (final AMap map : maps) {
        long mapped = map.apply(number);
        if (number != mapped) {
          return mapped;
        }
      }
      return number;
    }

    static Almanac fromBlock(String block) {
      List<String> lines = block.lines().toList();
      String[] fromTo = lines.get(0).split(" ")[0].split("-to-");

      // Parse maps
      return new Almanac(fromTo[0], fromTo[1], lines.stream().skip(1).map(AMap::fromLine).toList());
    }
  }

  public static void main(String[] args) throws IOException {
    List<String> blocks = Arrays.stream(Input.read("day05/input.txt").split("\n\n")).toList();
    List<Long> seeds = Arrays.stream(blocks.get(0).split(": ")[1].split(" ")).map(Long::parseLong).toList();
    List<Almanac> almanacs = blocks.stream().skip(1).map(Almanac::fromBlock).toList();

    var partOne = seeds.stream().map(seed -> {
      AtomicLong mappedSeed = new AtomicLong(seed);
//      System.out.print(STR. "Seed \{ seed }" );
      almanacs.forEach(almanac -> {
        mappedSeed.set(almanac.map(mappedSeed.get()));
//        System.out.print(STR. ", \{ almanac.to() } \{ mappedSeed.get() }" );
      });
//      System.out.println();
      return mappedSeed.get();
    }).min(Long::compare).get();

    var partTwo = Lists.partition(seeds, 2).stream().flatMapToLong(nums -> LongStream.range(nums.get(0), nums.get(0) + nums.get(1))).map(seed -> {
      AtomicLong mappedSeed = new AtomicLong(seed);
//      System.out.print(STR. "Seed \{ seed }" );
      almanacs.forEach(almanac -> {
        mappedSeed.set(almanac.map(mappedSeed.get()));
//        System.out.print(STR. ", \{ almanac.to() } \{ mappedSeed.get() }" );
      });
//      System.out.println();
      return mappedSeed.get();
    }).boxed().min(Long::compare).get();

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }
}
