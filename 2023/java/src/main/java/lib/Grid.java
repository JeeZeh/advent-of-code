package lib;

import com.google.common.base.Preconditions;

import java.util.Arrays;
import java.util.List;
import java.util.function.Function;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public interface Grid<T> {

  T[][] elements();

  default T getElement(int row, int col) {
    T[][] elements = elements();
    Preconditions.checkArgument(row >= 0 && row < elements.length,
        STR."Column '\{row}' not within bounds '0-\{elements.length}'");
    Preconditions.checkArgument(col >= 0 && col < elements[row].length,
        STR."Row '\{col}' not within bounds '0-\{elements[row].length}'");

    return elements[col][row];
  }

  default List<T[]> rows() {
    return Arrays.stream(elements()).toList();
  }

  default int width() {
    return elements()[0].length;
  }

  default int height() {
    return elements().length;
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
    Arrays.stream(elements()).forEach(row -> {
      Arrays.stream(row).forEach(sb::append);
      sb.append("\n");
    });
    return sb.toString();
  }
}
