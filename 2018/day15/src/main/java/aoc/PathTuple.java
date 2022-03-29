package aoc;

import java.util.Optional;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;

@AllArgsConstructor
@EqualsAndHashCode
public class PathTuple {
    final Point point;
    final Optional<Point> firstMove;
}
