package aoc;

import java.util.Objects;
import java.util.stream.Stream;
import lombok.AllArgsConstructor;

@AllArgsConstructor
public class Point implements Comparable<Point> {
    final int x, y;

    public int dist(Point other) {
        return Math.abs(other.x - this.x) + Math.abs(other.y - this.y);
    }

    public Point add(Point other) {
        return new Point(x + other.x, y + other.y);
    }

    public Stream<Point> getAdjacent() {
        return Stream.of(new Point(x, y - 1), new Point(x - 1, y), new Point(x + 1, y), new Point(x, y + 1));
    }

    /**
     * Compares two points in reading order, such that the following numbers indicate 
     * their sorted order when considered as a 2D grid of Points.
     * 
     * Example:
     *      1  2  3
     *      4 [5] 6  <- 5 is comparably greater than [1, 2, 3, 4] and less than [6, 7, 8, 9]
     *      7  8  9
     */
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

    @Override
    public String toString() {
        return "%d,%d".formatted(x, y);
    }
}
