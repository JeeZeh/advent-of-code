package aoc;

import java.util.List;
import java.util.Optional;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;


@Getter
@AllArgsConstructor
@EqualsAndHashCode
public class Entity implements Comparable<Entity> {
    final EntityType type;
    Point position;

    public Optional<Entity> findTargetInRange(List<Entity> entities) {
        var attackPositions = this.getPointsInRange();
        return entities
                .stream()
                .filter((Entity e) -> attackPositions.contains(e.position))
                .sorted()
                .findFirst();
    }

    public void move(Point direction) {
        this.position.add(direction);
    }

    public Set<Point> getPointsInRange() {
        return Stream
                .of(this.position.up(), this.position.down(), this.position.left(), this.position.right())
                .collect(Collectors.toSet());
    }

    public EntityType getTargetType() {
        return this.type == EntityType.Goblin ? EntityType.Elf : EntityType.Goblin;
    }

    @Override
    public int compareTo(Entity o) {
        return position.compareTo(o.position);
    }
}
