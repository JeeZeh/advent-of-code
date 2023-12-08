package day07;

import java.io.IOException;
import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.LongStream;
import lib.Input;



public class Solution {

  public static void main(String[] args) throws IOException {
    List<Hand> hands =
        new ArrayList<>(Input.lines("day07/input.txt").map(Hand::parseLine).sorted().toList());

    int partOne = 0;
    for (int i = 0; i < hands.size(); i++) {
      partOne += hands.get(i).bet * (i + 1);
    }

    System.out.println(STR. "Part 1: \{ partOne }" );
  }

  public record Hand(HandKind kind, List<Card> cards, int bet) implements Comparable<Hand> {
    static Hand parseLine(String line) {
      String[] parts = line.split(" ");
      List<Card> cards = parts[0].chars().mapToObj(i -> (char) i).map(Hand::getCard).toList();

      return new Hand(getKind(cards), cards, Integer.parseInt(parts[1]));
    }

    @Override
    public int compareTo(Hand o2) {
      var countByThis = this.cards.stream()
          .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
      var fakeCardsThis = cards.stream().toList();

      if (countByThis.containsKey(Card.J)) {
        Optional<Map.Entry<Card, Long>> jokerBecomes = countByThis.entrySet()
            .stream()
            .filter(e -> e.getKey() != Card.J)
            .max(Map.Entry.comparingByValue());
        if (jokerBecomes.isPresent()) {
          fakeCardsThis = fakeCardsThis.stream()
              .map(c -> c == Card.J ? jokerBecomes.get().getKey() : c)
              .toList();
        }
      }
      Hand fakeHandThis = new Hand(getKind(fakeCardsThis), fakeCardsThis, this.bet);


      var countByOther = o2.cards.stream()
          .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));
      var fakeCardsOther = o2.cards.stream().toList();

      if (countByOther.containsKey(Card.J)) {
        Optional<Map.Entry<Card, Long>> jokerBecomes = countByOther.entrySet()
            .stream()
            .filter(e -> e.getKey() != Card.J)
            .max(Map.Entry.comparingByValue());
        if (jokerBecomes.isPresent()) {
          fakeCardsOther = fakeCardsOther.stream()
              .map(c -> c == Card.J ? jokerBecomes.get().getKey() : c)
              .toList();

        }
      }

      Hand fakeHandOther = new Hand(getKind(fakeCardsOther), fakeCardsOther, o2.bet);

      int compareKind = fakeHandThis.kind.compareTo(fakeHandOther.kind);
      if (compareKind != 0) {
        return compareKind;
      }

      for (int i = 0; i < cards.size(); i++) {
        var compareHand = this.cards.get(i).compareTo(o2.cards.get(i));
        if (compareHand != 0) {
          return compareHand;
        }
      }

      return 0;
    }



    public enum HandKind {
      HIGH,
      ONE,
      TWO,
      THREE,
      FULL,
      FOUR,
      FIVE,
    }


    public enum Card {
      J,
      TWO,
      THREE,
      FOUR,
      FIVE,
      SIX,
      SEVEN,
      EIGHT,
      NINE,
      T,
      Q,
      K,
      A,
    }

    static Card getCard(char c) {
      return switch (c) {
        case 'A' -> Card.A;
        case 'K' -> Card.K;
        case 'Q' -> Card.Q;
        case 'J' -> Card.J;
        case 'T' -> Card.T;
        case '9' -> Card.NINE;
        case '8' -> Card.EIGHT;
        case '7' -> Card.SEVEN;
        case '6' -> Card.SIX;
        case '5' -> Card.FIVE;
        case '4' -> Card.FOUR;
        case '3' -> Card.THREE;
        case '2' -> Card.TWO;
        default -> throw new RuntimeException(STR. "Cannot map char \{ c } to Card" );
      };
    }

    static HandKind getKind(List<Card> cards) {
      var countBy =
          cards.stream().collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

      if (countBy.keySet().size() == 1) {
        return HandKind.FIVE;
      }

      if (countBy.keySet().size() == 2) {
        if (countBy.values().stream().anyMatch(vals -> vals == 4) && countBy.values()
            .stream()
            .anyMatch(vals -> vals == 1)) {
          return HandKind.FOUR;
        }
        if (countBy.values().stream().anyMatch(vals -> vals == 3) && countBy.values()
            .stream()
            .anyMatch(vals -> vals == 2)) {
          return HandKind.FULL;
        }
      }

      if (countBy.keySet().size() == 3) {
        if (countBy.values().stream().anyMatch(vals -> vals == 3) && countBy.values()
            .stream()
            .anyMatch(vals -> vals == 1)) {
          return HandKind.THREE;
        }

        return HandKind.TWO;
      }
      if (countBy.keySet().size() == 4) {
        return HandKind.ONE;
      }

      return HandKind.HIGH;
    }
  }

}
