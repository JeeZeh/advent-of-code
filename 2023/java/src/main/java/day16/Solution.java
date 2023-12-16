package day16;

import day13.Solution.Pattern;
import java.io.IOException;
import java.util.List;
import lib.Grid;
import lib.Input;

public class Solution {

  public static void main(String[] args) throws IOException {
    Contraption contraption = Contraption.fromLines(Input.lines("day16/example.txt").toList());
    System.out.println(contraption.asString());
  }


  record Contraption(List<List<Tile>> elements) implements Grid<Tile> {

    static Contraption fromLines(List<String> lines) {
      return new Contraption(
          lines.stream().map(line -> line.chars().mapToObj(c -> Tile.fromChar((char) c)).toList())
              .toList());
    }
  }

  enum Tile {
    EMPTY, MIRROR_RIGHT, MIRROR_LEFT, SPLITTER_VERT, SPLITTER_HORIZ;

    static Tile fromChar(char c) {
      return switch (c) {
        case '.' -> EMPTY;
        case '/' -> MIRROR_RIGHT;
        case '\\' -> MIRROR_LEFT;
        case '-' -> SPLITTER_HORIZ;
        case '|' -> SPLITTER_VERT;
        default -> throw new IllegalStateException(STR."Unexpected value: \{c}");
      };
    }

    @Override
    public String toString() {
      return switch (this) {
        case EMPTY -> ".";
        case MIRROR_RIGHT -> "/";
        case MIRROR_LEFT -> "\\";
        case SPLITTER_HORIZ -> "-";
        case SPLITTER_VERT -> "|";
      };
    }
  }

}
