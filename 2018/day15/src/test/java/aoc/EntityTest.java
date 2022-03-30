package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import java.util.Arrays;
import java.util.Optional;
import org.junit.Test;

public class EntityTest {
    @Test
    public void testGetPointsInRange() {
        Entity entity = new Entity(EntityType.Goblin, new Point(0, 0));

        var inRange = entity.getPointsInRange();
        assertTrue(inRange.contains(new Point(1, 0)));
        assertTrue(inRange.contains(new Point(-1, 0)));
        assertTrue(inRange.contains(new Point(0, 1)));
        assertTrue(inRange.contains(new Point(0, -1)));
        assertEquals(inRange.size(), 4);
    }

    @Test
    public void testFindTargetsInReadingOrder() {
        var activeEntity = new Entity(EntityType.Goblin, new Point(0, 0));
        var shouldFind = new Entity(EntityType.Elf, new Point(0, -1));
        var entities = Arrays.asList(new Entity(EntityType.Elf, new Point(-1, 0)), shouldFind);
        var target = activeEntity.findTargetInRange(entities);

        assertTrue(target.isPresent());
        assertEquals(target.get(), shouldFind);
    }

    @Test
    public void testFindTargetFindsEnemy() {
        var activeEntity = new Entity(EntityType.Goblin, new Point(0, 0));
        var shouldFind = new Entity(EntityType.Elf, new Point(0, 1));
        var entities = Arrays.asList(shouldFind);
        var target = activeEntity.findTargetInRange(entities).get();

        assertEquals(target, shouldFind);
    }

    @Test
    public void testFindTargetByLowestHp() {
        var activeEntity = new Entity(EntityType.Goblin, new Point(0, 0));
        var shouldFind = new Entity(EntityType.Elf, new Point(1, 0));
        shouldFind.hp = 100;
        var entities = Arrays.asList(shouldFind, new Entity(EntityType.Elf, new Point(-1, 0)));
        var target = activeEntity.findTargetInRange(entities).get();

        assertEquals(target, shouldFind);
    }

    @Test
    public void testFindTargetByLowestHpResolvesByReadingOrder() {
        var activeEntity = new Entity(EntityType.Goblin, new Point(0, 0));
        var shouldFind = new Entity(EntityType.Elf, new Point(-1, 0));
        var entities = Arrays.asList(shouldFind, new Entity(EntityType.Elf, new Point(1, 0)));
        var target = activeEntity.findTargetInRange(entities).get();

        assertEquals(target, shouldFind);
    }

    @Test
    public void testToString() {
        var test = new Entity(EntityType.Goblin, new Point(-1, 20));
        assertEquals("Goblin @ -1,20", test.toString());
    }

    @Test
    public void testFindNearestReachablePositionEasy() {
        Cave world = Cave.fromString("#######\n#E..G.#\n#...#.#\n#.G.#G#\n#######");
        var entity = world.entities.get(0);
        assertEquals("Elf @ 1,1", entity.toString());

        Optional<Point> reachable = entity.tryGetNextMovement(world);
        assertTrue(reachable.isPresent());
        assertEquals(new Point(2, 1), reachable.get());
    }

    @Test
    public void testFindNearestReachablePositionHard() {
        Cave world = Cave.fromString("#######\n#E....#\n#...#.#\n#...#G#\n#######");
        var entity = world.entities.get(0);
        assertEquals("Elf @ 1,1", entity.toString());

        Optional<Point> reachable = entity.tryGetNextMovement(world);
        assertTrue(reachable.isPresent());
        assertEquals(new Point(2, 1), reachable.get());
    }

    @Test
    public void testFindNearestReachablePositionImpossibleWall() {
        Cave world = Cave.fromString("#######\n#E..#.#\n#...#.#\n#...#G#\n#######");
        var entity = world.entities.get(0);
        assertEquals("Elf @ 1,1", entity.toString());

        Optional<Point> reachable = entity.tryGetNextMovement(world);
        assertTrue(reachable.isEmpty());
    }

    @Test
    public void testFindNearestReachablePositionImpossibleEntity() {
        Cave world = Cave.fromString("#######\n#E..E.#\n#...#.#\n#...#G#\n#######");
        var entity = world.entities.get(0);
        assertEquals("Elf @ 1,1", entity.toString());

        Optional<Point> reachable = entity.tryGetNextMovement(world);
        assertTrue(reachable.isEmpty());
    }
}
