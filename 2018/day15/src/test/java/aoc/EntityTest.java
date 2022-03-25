package aoc;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;
import java.util.Arrays;
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
        var target = activeEntity.findTargetInRange(entities);

        assertTrue(target.isPresent());
        assertEquals(target.get(), shouldFind);
    }
}
