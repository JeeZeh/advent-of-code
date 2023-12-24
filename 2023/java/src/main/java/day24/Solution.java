package day24;

import java.awt.geom.Point2D;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import lib.Input;

public class Solution {

  public record Hail(long x, long y, long z, long vx, long vy, long vz) {

    public boolean isFuture(Point2D.Double point) {
      return (vx > 0 && point.x > x || vx < 0 && point.x < x) && (vy > 0 && point.y > y
          || vy < 0 && point.y < y);
    }

    public double getSlope2D() {
      return (double) vy / (double) vx;
    }

    public Optional<Point2D.Double> getIntersection(Hail other) {
      double m1 = getSlope2D();
      double m2 = other.getSlope2D();

      // Check if the slopes are equal, lines are parallel, and won't intersect
      if (Double.compare(m1, m2) == 0) {
        // Lines are parallel, no intersection point
        return Optional.empty();
      }

      // Calculate the intersection point
      double xIntersect = (other.y - y + m1 * x - m2 * other.x) / (m1 - m2);
      double yIntersect = m1 * (xIntersect - x) + y;
      var point = new Point2D.Double();
      point.setLocation(xIntersect, yIntersect);
      return Optional.of(point);
    }


    static Hail fromLine(String line) {
      var parts = line.split(" @ ");
      var left = Arrays.stream(parts[0].split(", ")).map(Long::parseLong).toList();
      var right = Arrays.stream(parts[1].split(", ")).map(Long::parseLong).toList();

      return new Hail(left.get(0), left.get(1), left.get(2), right.get(0), right.get(1),
          right.get(2));
    }
  }

  static long partOne(List<Hail> hails, long boundFrom, long boundTo) {
    List<Point2D.Double> points = new ArrayList<>();
    for (int i = 0; i < hails.size() - 1; i++) {
      for (int j = i + 1; j < hails.size(); j++) {
        var a = hails.get(i);
        var b = hails.get(j);
        a.getIntersection(b).ifPresent(intersect -> {
          if (a.isFuture(intersect) && b.isFuture(intersect)) {
            points.add(intersect);
          }
        });


      }
    }

    return points.stream()
        .filter(i -> i.x >= boundFrom && i.y >= boundFrom && i.x <= boundTo && i.y <= boundTo)
        .count();
  }

  public static void main(String[] args) throws IOException {
    boolean example = false;
    List<Hail> hail = Input.lines(example ? "day24/example.txt" : "day24/input.txt")
        .map(Hail::fromLine).toList();

    long from = example ? 7 : 200000000000000L;
    long to = example ? 27 : 400000000000000L;

    System.out.println(STR."Part 1: \{partOne(hail, from, to)}");
    long partTwo = 0;
    System.out.println(STR."Part 2: \{partTwo}");
  }


}
