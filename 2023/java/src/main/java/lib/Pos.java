package lib;

import java.util.Optional;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public record Pos(int x, int y) {

  public enum Direction {
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT;

    public static Optional<Direction> getDir(Pos a, Pos b) {
      if (a.x == b.x) {
        if (a.y < b.y) {
          return Optional.of(DOWN);
        }
        if (a.y > b.y) {
          return Optional.of(UP);
        }
      }
      if (a.x < b.x) {
        return Optional.of(RIGHT);
      }
      if (a.x > b.y) {
        return Optional.of(LEFT);
      }

      return Optional.empty();
    }
  }

  public Pos add(Pos other) {
    return new Pos(this.x + other.x, this.y + other.y);
  }

  public Pos add(int x, int y) {
    return new Pos(this.x + x, this.y + y);
  }

  boolean isWithin(Pos min, Pos max) {
    return x >= min.x && x <= max.x && y >= min.y && y <= max.y;
  }

  /**
   * Returns neighbouring positions surrounding this Pos.
   */
  public Stream<Pos> neighbours() {
    return neighbours(this);
  }

  /**
   * Returns neighbouring positions surrounding this Pos (start) and the provided Pos (end).
   */
  Stream<Pos> neighbours(Pos end) {
    return IntStream.range(this.y - 1, end.y + 2).boxed()
        .flatMap(y -> IntStream.range(this.x - 1, end.x + 2).boxed().map(x -> new Pos(x, y)));
  }
}
