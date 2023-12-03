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
        Preconditions.checkArgument(row >= 0 && row < elements.length, STR. "Column '\{ row }' not within bounds '0-\{ elements.length }'" );
        Preconditions.checkArgument(col >= 0 && col < elements[row].length, STR. "Row '\{ col }' not within bounds '0-\{ elements[row].length }'" );

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

    default Stream<Pos> surroundingPositions(Pos pos) {
        return surroundingPositions(pos, pos);
    }

    default Stream<Pos> surroundingPositions(Pos start, Pos end) {
        int minX = Math.max(start.x() - 1, 0);
        int minY = Math.max(start.y() - 1, 0);
        int maxX = Math.min(end.x() + 1, width() - 1);
        int maxY = Math.min(end.y() + 1, height() - 1);

        return IntStream.range(minY, maxY + 1).boxed().flatMap(y -> IntStream.range(minX, maxX + 1).boxed().map(x -> new Pos(x, y)));
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
