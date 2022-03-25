package aoc;

import java.util.Objects;
import lombok.AllArgsConstructor;

@AllArgsConstructor
public class Point implements Comparable<Point> {
    final int x, y;

    public Point add(Point other) {
        return new Point(x + other.x, y + other.y);
    }

    public Point up() {
        return new Point(x, y - 1);
    }

    public Point down() {
        return new Point(x, y + 1);
    }

    public Point left() {
        return new Point(x - 1, y);
    }

    public Point right() {
        return new Point(x + 1, y);
    }

    @Override
    public int compareTo(Point other) {
        if (y < other.y) {
            return -1;
        }
        if (y > other.y) {
            return 1;
        }
        if (y == other.y) {
            if (x < other.x) {
                return -1;
            }
            if (x > other.x) {
                return 1;
            }
        }

        return 0;
    }

    @Override
    public boolean equals(Object o) {
        if (o instanceof Point) {
            Point p = (Point) o;
            return p.x == x && p.y == y;
        }
        return false;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}
