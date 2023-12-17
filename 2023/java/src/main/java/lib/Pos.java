package lib;

import java.util.Optional;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public record Pos(int x, int y) implements Comparable<Pos> {


  public long dist(Pos b) {
    return Math.abs(b.x - this.x) + Math.abs(b.y - this.y);
  }

  @Override
  public int compareTo(Pos o) {
    if (this.y == o.y) {
      return Integer.compare(this.x, o.x);
    }

    return Integer.compare(this.y, o.y);
  }

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
      if (a.y == b.y) {
        if (a.x < b.x) {
          return Optional.of(RIGHT);
        }
        if (a.x > b.x) {
          return Optional.of(LEFT);
        }
      }

      return Optional.empty();
    }

    public Pos transpose(Pos current) {
      return switch (this) {
        case UP -> current.add(0, -1);
        case DOWN -> current.add(0, 1);
        case LEFT -> current.add(-1, 0);
        case RIGHT -> current.add(1, 0);
        case UP_LEFT -> current.add(-1, -1);
        case UP_RIGHT -> current.add(1, -1);
        case DOWN_LEFT -> current.add(-1, 1);
        case DOWN_RIGHT -> current.add(1, 1);
      };
    }

    public Pos transpose(Pos current, int times) {
      return switch (this) {
        case UP -> current.add(0, -times);
        case DOWN -> current.add(0, times);
        case LEFT -> current.add(-times, 0);
        case RIGHT -> current.add(times, 0);
        case UP_LEFT -> current.add(-times, -times);
        case UP_RIGHT -> current.add(times, -times);
        case DOWN_LEFT -> current.add(-times, times);
        case DOWN_RIGHT -> current.add(times, times);
      };
    }

    public Direction invert() {
      return switch (this) {
        case UP -> DOWN;
        case DOWN -> UP;
        case LEFT -> RIGHT;
        case RIGHT -> LEFT;
        case UP_LEFT -> DOWN_RIGHT;
        case UP_RIGHT -> DOWN_LEFT;
        case DOWN_LEFT -> UP_RIGHT;
        case DOWN_RIGHT -> UP_LEFT;
      };
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
