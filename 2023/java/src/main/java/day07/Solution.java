package day07;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.function.Function;
import java.util.stream.Collectors;
import lib.Input;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<Hand> hands =
        new ArrayList<>(Input.lines("day07/input.txt").map(Hand::parseLine).sorted().toList());

    int partOne = 0;
    for (int i = 0; i < hands.size(); i++) {
      partOne += hands.get(i).bet * (i + 1);
    }

    hands.sort(Hand::compareToFake);
    int partTwo = 0;
    for (int i = 0; i < hands.size(); i++) {
      partTwo += hands.get(i).bet * (i + 1);
    }

    System.out.println(STR. "Part 1: \{ partOne }" );
    System.out.println(STR. "Part 2: \{ partTwo }" );
  }

  public record Hand(HandKind kind, List<Card> cards, int bet) implements Comparable<Hand> {
    static Hand parseLine(String line) {
      String[] parts = line.split(" ");
      List<Card> cards = parts[0].chars().mapToObj(i -> (char) i).map(Hand::getCard).toList();

      return new Hand(getKind(cards), cards, Integer.parseInt(parts[1]));
    }

    public Hand fakeHand() {
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
      return new Hand(getKind(fakeCardsThis), fakeCardsThis, this.bet);
    }

    public int compareToFake(Hand o2) {
      Hand fakeThis = this.fakeHand();
      Hand fakeOther = o2.fakeHand();
      int compareKind = fakeThis.kind.compareTo(fakeOther.kind);
      if (compareKind != 0) {
        return compareKind;
      }

      for (int i = 0; i < fakeThis.cards.size(); i++) {
        var compareHand = fakeThis.cards.get(i).compareToFake(fakeOther.cards.get(i));
        if (compareHand != 0) {
          return compareHand;
        }
      }

      return 0;
    }

    @Override
    public int compareTo(Hand o2) {
      int compareKind = this.kind.compareTo(o2.kind);
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
      TWO,
      THREE,
      FOUR,
      FIVE,
      SIX,
      SEVEN,
      EIGHT,
      NINE,
      T,
      J,
      Q,
      K,
      A;

      int compareToFake(Card o2) {
        if (this == Card.J) {
          if (o2 == Card.J) {
            return 0;
          }
          return -1;
        } else {
          if (o2 == Card.J) {
            return 1;
          }
        }

        return this.compareTo(o2);
      }
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
