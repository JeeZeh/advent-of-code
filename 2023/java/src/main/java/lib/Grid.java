package lib;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.IntStream;
import java.util.stream.Stream;


public interface Grid<T> {

  enum RotationDirection {
    CLOCKWISE, COUNTER_CLOCKWISE
  }

  List<List<T>> elements();

  default List<List<T>> rotate(RotationDirection direction) {
    var elements = elements();
    if (elements.isEmpty() || elements.get(0).isEmpty()) {
      return elements; // No rotation for empty matrix
    }

    int rows = elements.size();
    int cols = elements.get(0).size();

    List<List<T>> rotatedMatrix = new ArrayList<>(cols);
    for (int i = 0; i < cols; i++) {
      rotatedMatrix.add(new ArrayList<>(rows));
    }

    if (direction == RotationDirection.CLOCKWISE) {
      for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
          rotatedMatrix.get(j).add(0, elements.get(i).get(j));
        }
      }
    } else if (direction == RotationDirection.COUNTER_CLOCKWISE) {
      for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
          rotatedMatrix.get(cols - 1 - j).add(elements.get(i).get(j));
        }
      }
    } else {
      throw new IllegalArgumentException("Invalid rotation direction");
    }

    return rotatedMatrix;
  }

  default List<List<T>> mirror(boolean horizontal, boolean vertical) {
    if (!horizontal && !vertical) {
      return elements();
    }

    var temp = elements().stream().map(row -> horizontal ? row.reversed() : row).toList();
    if (vertical) {
      return temp.reversed();
    }

    return temp;
  }

  default int width() {
    return elements().getFirst().size();
  }

  default int height() {
    return elements().size();
  }

  default Stream<Pos> surroundingPositions(Pos start) {
    int minX = Math.max(0, start.x() - 1);
    int minY = Math.max(0, start.y() - 1);
    int maxX = Math.min(width() - 1, start.x() + 1);
    int maxY = Math.min(height() - 1, start.y() + 1);

    return IntStream.range(minY, maxY + 1).boxed()
        .flatMap(y -> IntStream.range(minX, maxX + 1).boxed().map(x -> new Pos(x, y)))
        .filter(p -> !p.equals(start));
  }

  default String asString() {
    StringBuilder sb = new StringBuilder();
    elements().forEach(row -> {
      row.forEach(sb::append);
      sb.append("\n");
    });
    return sb.toString();
  }
}
