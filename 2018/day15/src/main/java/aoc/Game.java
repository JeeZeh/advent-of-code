package aoc;

import java.util.Optional;

public class Game {
    final Cave cave;
    boolean friendlyMode;

    public Game(Cave cave, boolean friendlyMode) {
        this.cave = cave;
        this.friendlyMode = friendlyMode;
    }

    public void setFriendlyMode(boolean friendlyMode) {
        this.friendlyMode = friendlyMode;
    }

    public Optional<Game> step() {
        boolean stale = true;

        for (final Entity takingTurn : cave.entities.stream().sorted().toList()) {
            // Since we're looping through a snapshot of the entities as they were
            // at the start of the round, we should check if the entity is dead before
            // giving it a turn
            if (takingTurn.hp <= 0) {
                continue;
            }
            var targets = cave.entities.stream().filter((Entity e) -> e.type != takingTurn.type).toList();
            if (targets.size() == 0) {
                stale = true;
                break;
            }

            // Try attack
            var toAttack = takingTurn.findTargetInRange(cave.entities);
            if (toAttack.isEmpty()) {
                // Move
                var nextMove = takingTurn.tryGetNextMovement(cave);
                if (nextMove.isPresent()) {
                    stale = false;
                    takingTurn.moveTo(nextMove.get());
                }
                // Try find a target again
                toAttack = takingTurn.findTargetInRange(cave.entities);
            }

            if (!friendlyMode && toAttack.isPresent()) {
                final var target = toAttack.get();

                // Check if we killed the target
                if (takingTurn.attack(target)) {
                    // Remove it from play
                    cave.entities.removeIf((Entity e) -> e.equals(target));
                }
            }
        }
        if (stale) {
            return Optional.empty();
        }
        return Optional.of(this);
    }

    @Override
    public String toString() {
        return cave.toString();
    }
}
