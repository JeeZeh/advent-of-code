package lib;

import java.util.stream.IntStream;
import java.util.stream.Stream;

public record Pos(int x, int y) {

  boolean isWithin(Pos min, Pos max) {
    return x >= min.x && x <= max.x && y >= min.y && y <= max.y;
  }

  /**
   * Returns neighbouring positions surrounding this Pos.
   */
  Stream<Pos> neighbours() {
    return neighbours(this);
  }

  /**
   * Returns neighbouring positions surrounding this Pos (start) and the provided Pos (end).
   */
  Stream<Pos> neighbours(Pos end) {
    return IntStream.range(this.y - 1, end.y + 2).boxed().flatMap(y -> IntStream.range(this.x - 1, end.x + 2).boxed().map(x -> new Pos(x, y)));
  }
}
