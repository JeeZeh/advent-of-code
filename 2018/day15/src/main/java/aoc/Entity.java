package aoc;

import java.util.ArrayDeque;
import java.util.HashSet;
import java.util.List;
import java.util.Optional;
import java.util.Queue;
import java.util.Set;
import java.util.stream.Collectors;
import lombok.EqualsAndHashCode;
import lombok.Getter;

@Getter
@EqualsAndHashCode
public class Entity implements Comparable<Entity> {
    final int AP = 3;
    final EntityType type;
    Point position;
    int hp = 200;

    public Entity(EntityType type, Point position) {
        this.type = type;
        this.position = position;
    }

    public Optional<Point> tryGetNextMovement(Cave cave) {
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
                return nextTuple.firstMove;
            }
            if (found.isEmpty()) {
                nextTuple.point.getAdjacent()
                        .filter((Point p) -> !seen.contains(p) && cave.isFloor(p) && !entityLocations.contains(p))
                        .forEach((Point p) -> {
                            seen.add(p);
                            toExplore.add(new PathTuple(p, nextTuple.firstMove.or(() -> {
                                return Optional.of(p);
                            })));
                        });
            }
        }

        return Optional.empty();
    }

    public Optional<Entity> findTargetInRange(List<Entity> entities) {
        var attackPositions = this.getPointsInRange();
        // Sort targets by HP and then position if tied
        return entities
                .stream()
                .filter((Entity e) -> attackPositions.contains(e.position))
                .sorted(Entity::compareByHpAndPosition)
                .findFirst();
    }

    public void moveTo(Point position) {
        this.position = position;
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

    public static int compareByHpAndPosition(Entity a, Entity b) {
        int hpCompare = a.hp - b.hp;
        return hpCompare != 0 ? hpCompare : a.compareTo(b);
    }

    @Override
    public int compareTo(Entity o) {
        return position.compareTo(o.position);
    }

    @Override
    public String toString() {
        return "%s @ %s".formatted(type, position);
    }

    public boolean attack(Entity target) {
        target.hp -= this.AP;
        return target.hp <= 0;
    }
}
