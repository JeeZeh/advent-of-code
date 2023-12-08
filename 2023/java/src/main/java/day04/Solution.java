package day04;

import lib.Input;

import java.io.IOException;
import java.math.BigInteger;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

public class Solution {

  public record Card(int id, List<Integer> winning, List<Integer> have, int points, int copies) {


    static Card fromLine(String line) {
      String[] parts = line.split(": ");
      int id = Integer.parseInt(parts[0].substring("Card".length()).trim());
      String[] sets = parts[1].split(" \\| ");
      List<Integer> winning = Arrays.stream(sets[0].split("\\s+")).filter(e -> !e.isEmpty())
          .map(Integer::parseInt).toList();
      List<Integer> have = Arrays.stream(sets[1].split("\\s+")).filter(e -> !e.isEmpty())
          .map(Integer::parseInt).toList();

      int matching = (int) have.stream().filter(winning::contains).count();
      return new Card(id, winning, have, (int) Math.pow(2, matching - 1), matching);
    }

    String asString() {
      return STR. "Card \{ id }: \{ Arrays.toString(winning.toArray()) } | \{ Arrays.toString(
          have.toArray()) }" ;
    }
  }

  public static void main(String[] args) throws IOException {
    List<Card> cards = Input.lines("day04/input.txt").map(Card::fromLine).toList();
    int partOne = cards.stream().mapToInt(Card::points).sum();
    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo(cards) }" );
  }

  /**
   * Part two sucked because I forgot that points in part one weren't just the number of matches,
   * but a power of 2. So I was adding (incorrectly) the original points from part one, instead of
   * just the number of matches found for a given card.
   */
  public static BigInteger partTwo(List<Card> cards) {
    final BigInteger[] cardCounts = new BigInteger[cards.size()];
    for (int i = 0; i < cards.size(); i++) {
      cardCounts[i] = BigInteger.ONE;
    }

    for (int i = 0; i < cards.size() - 1; i++) {
      int until = Math.min(cards.size(), 1 + i + cards.get(i).copies);
      int finalI = i;
      IntStream.range(i + 1, until)
          .forEach(copy -> cardCounts[copy] = cardCounts[copy].add(cardCounts[finalI]));
    }
    return Arrays.stream(cardCounts).reduce((v, acc) -> acc.add(v)).get();
  }
}
