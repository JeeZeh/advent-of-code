package day18;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.function.BiFunction;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  public static void main(String[] args) throws IOException {
    Lagoon city = Lagoon.fromDigPlan(Input.lines("day18/input.txt").toList());
    long partOne = city.lavaPoolSize();
    int partTwo = 0;
    System.out.println(STR."Part 1: \{partOne}");
    ;
  }


  record Lagoon(List<List<Tile>> elements, Map<Pos, Tile> digMap) implements Grid<Tile> {

    long lavaPoolSize() {
      Pos probe = new Pos(0, 1);
      // Move until dig
      while (probe.x() < width() && get(probe) == Tile.EMPTY) {
        probe = Direction.RIGHT.transpose(probe);
      }
      // Move until gap
      while (probe.x() < width() && get(probe) != Tile.EMPTY) {
        probe = Direction.RIGHT.transpose(probe);
      }

      Set<Pos> lavaPool = new HashSet<>(digMap.keySet());
      BiFunction<Integer, Integer, String> poolMapper = (x, y) -> {
        var test = new Pos(x, y);
        return lavaPool.contains(test) && !digMap.containsKey(test) ? "*" : get(test).toString();
      };

      Deque<Pos> nextPositions = new ArrayDeque<>();
      nextPositions.add(probe);
      while (!nextPositions.isEmpty()) {
        var curr = nextPositions.poll();

        surroundingPositions(curr).forEach(next -> {
          if (!lavaPool.contains(next)) {
            lavaPool.add(next);
            nextPositions.add(next);
          }
        });
//        System.out.println(this.asString(poolMapper));
      }
      return lavaPool.size();
    }

    static Lagoon fromDigPlan(List<String> lines) {
      var digMap = getPosStringMap(lines);
      var minX = digMap.keySet().stream().mapToInt(Pos::x).min().getAsInt();
      var minY = digMap.keySet().stream().mapToInt(Pos::y).min().getAsInt();
      var maxX = digMap.keySet().stream().mapToInt(Pos::x).max().getAsInt();
      var maxY = digMap.keySet().stream().mapToInt(Pos::y).max().getAsInt();
      var width = maxX - minX + 1;
      var height = maxY - minY + 1;

      Map<Pos, Tile> newDigMap = new HashMap<>();
      List<List<Tile>> elements = new ArrayList<>(height);
      for (int y = 0; y < height; y++) {
        List<Tile> row = new ArrayList<>(width);
        for (int x = 0; x < width; x++) {
          var tile = digMap.get(new Pos(x + minX, y + minY));
          if (tile != null) {
            newDigMap.put(new Pos(x, y), tile);
          }
          row.add(tile == null ? Tile.EMPTY : tile);
        }
        elements.add(row);
      }

      return new Lagoon(elements, newDigMap);
    }

    private static Map<Pos, Tile> getPosStringMap(List<String> lines) {
      Map<Pos, Tile> lagoon = new HashMap<>();

      Pos current = new Pos(0, 0);
      for (String row : lines) {
        String[] parts = row.split(" ");
        var direction = switch (parts[0]) {
          case "R" -> Direction.RIGHT;
          case "U" -> Direction.UP;
          case "D" -> Direction.DOWN;
          case "L" -> Direction.LEFT;
          default -> throw new IllegalStateException(STR."Unexpected value: \{parts[0]}");
        };

        var dist = Integer.parseInt(parts[1]);
        var color = parts[2].substring(1, parts[2].length() - 2);
        if (current.equals(new Pos(0, 0))) {
          lagoon.put(current, Tile.DIG.setColor(color));
        }
        for (int i = 0; i < dist; i++) {
          current = direction.transpose(current);
          lagoon.put(current, Tile.DIG.setColor(color));
        }
      }
      return lagoon;
    }
  }

  enum Tile {
    EMPTY("."), DIG(".");

    public String color = "";

    private Tile(String color) {
      this.color = color;
    }

    public Tile setColor(String color) {
      this.color = color;
      return this;
    }

    @Override
    public String toString() {
      return switch (this) {
        case EMPTY -> ".";
        case DIG -> "#";
      };
    }
  }
}
