package aoc;

import java.util.ArrayDeque;
import java.util.HashSet;
import java.util.List;
import java.util.Optional;
import java.util.Queue;
import java.util.Set;
import java.util.stream.Collectors;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;

@Getter
@AllArgsConstructor
@EqualsAndHashCode
public class Entity implements Comparable<Entity> {
    final EntityType type;
    Point position;

    public Optional<Point> findNearestReachablePosition(Cave cave) {
        Set<Point> entityLocations = cave.entities.stream().map(Entity::getPosition).collect(Collectors.toSet());
        Set<Point> validLocations = cave.entities.stream().filter(this::isEnemy)
                .flatMap((Entity e) -> e.position.getAdjacent()).collect(Collectors.toSet());
        Queue<PathTuple> toExplore = new ArrayDeque<>();
        toExplore.add(new PathTuple(position, Optional.empty()));
        Set<Point> seen = new HashSet<>();
        Set<PathTuple> found = new HashSet<>();
        seen.add(this.position);
        while (!toExplore.isEmpty()) {
            var nextTuple = toExplore.remove();
            if (validLocations.contains(nextTuple.point)) {
                found.add(nextTuple);
            }
            if (found.isEmpty()) {
                nextTuple.point.getAdjacent()
                        .filter((Point p) -> !seen.contains(p) && cave.isFloor(p) && !entityLocations.contains(p))
                        .forEach((Point p) -> {
                            seen.add(p);
                            toExplore.add(new p);
                        });
            }
        }

        return found.stream().sorted().findFirst();
    }

    public Optional<Entity> findTargetInRange(List<Entity> entities) {
        var attackPositions = this.getPointsInRange();
        return entities
                .stream()
                .filter((Entity e) -> attackPositions.contains(e.position))
                .sorted()
                .findFirst();
    }

    public void moveTowards(Point direction) {
        if (direction.y < position.y) {
            this.position = this.position.add(new Point(0, -1));
        } else if (direction.x < position.x) {
            this.position = this.position.add(new Point(-1, 0));
        } else if (direction.x > position.x) {
            this.position = this.position.add(new Point(1, 0));
        } else if (direction.y > position.y) {
            this.position = this.position.add(new Point(0, 1));
        }
    }

    public Set<Point> getPointsInRange() {
        return this.position.getAdjacent().collect(Collectors.toSet());
    }

    public boolean isEnemy(Entity other) {
        return !other.equals(this) && other.type != this.type;
    }

    public EntityType getTargetType() {
        return this.type == EntityType.Goblin ? EntityType.Elf : EntityType.Goblin;
    }

    @Override
    public int compareTo(Entity o) {
        return position.compareTo(o.position);
    }

    @Override
    public String toString() {
        return "%s @ %s".formatted(type, position);
    }
}
