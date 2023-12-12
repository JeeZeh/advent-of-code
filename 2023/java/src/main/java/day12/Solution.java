package day12;

import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import lib.Input;
import org.apache.commons.lang3.ArrayUtils;

/**
 * This solution is a re-implementation of Eoin Davey's solution in Python.
 * <p>
 * I didn't want to work through today's (not a fan of combinatorial problems), so I chose the
 * challenge of understanding and converting an existing Python solution to Java.
 */
public class Solution {

  public record Pair<T>(T a, T b) {

  }

  public record Schema(char[] pattern, int[] groups, Map<Pair<Integer>, Long> countCache,
                       Map<Pair<Integer>, Boolean> fitCache) {

    public long count(int position, int groupIdx) {
      // See if the last group can fit anywhere
      if (groupIdx == groups.length) {
        return IntStream.range(position, pattern.length).allMatch(this::spaceOrDamaged) ? 1 : 0;
      }

      // End of pattern
      if (position >= pattern.length) {
        return 0;
      }

      // Have we evaluated this group against from this point on already?
      var countState = new Pair<Integer>(position, groupIdx);
      var memoed = countCache.get(countState);
      if (memoed != null) {
        return memoed;
      }

      long count = 0;
      if (fits(position, groups[groupIdx])) {
        count += count(position + groups[groupIdx] + 1, groupIdx + 1);
      }
      if (spaceOrDamaged(position)) {
        count += count(position + 1, groupIdx);
      }
      countCache.put(countState, count);

      return count;
    }

    boolean fits(int from, int splitSize) {
      var fitState = new Pair<Integer>(from, splitSize);
      var fitResult = fitCache.get(fitState);
      if (fitResult != null) {
        return fitResult;
      }
      int to = from + splitSize;
      if (to > pattern.length) {
        return false;
      }

      boolean canPlace = IntStream.range(from, to).allMatch(this::springOrDamaged);
      boolean placeAfter = to == pattern.length || spaceOrDamaged(to);
      fitResult = canPlace && placeAfter;
      fitCache.put(fitState, fitResult);
      return fitResult;
    }

    boolean spaceOrDamaged(int position) {
      return pattern[position] == '.' || pattern[position] == '?';
    }

    boolean springOrDamaged(int position) {
      return pattern[position] == '#' || pattern[position] == '?';
    }

    public Schema unfold() {
      var concatSchema = IntStream.range(0, 5).mapToObj(x -> new String(pattern))
          .collect(Collectors.joining("?")).toCharArray();
      var concatGroups = IntStream.range(0, 5).mapToObj(x -> groups).reduce(ArrayUtils::addAll)
          .get();
      return new Schema(concatSchema, concatGroups, new HashMap<>(), new HashMap<>());
    }

    static Schema fromLine(String line) {
      var parts = line.split(" ");
      return new Schema(parts[0].toCharArray(),
          Arrays.stream(parts[1].split(",")).mapToInt(Integer::parseInt).toArray(), new HashMap<>(),
          new HashMap<>());
    }
  }

  public static void main(String[] args) throws IOException {
    List<Schema> originalSchemas = Input.lines("day12/input.txt").map(Schema::fromLine).toList();
    long partOne = originalSchemas.stream().mapToLong(schema -> schema.count(0, 0)).sum();
    long partTwo = originalSchemas.stream().mapToLong(schema -> schema.unfold().count(0, 0)).sum();

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }
}
