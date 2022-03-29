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
            var targets = cave.entities.stream().filter((Entity e) -> e.type != takingTurn.type).toList();
            if (targets.size() == 0) {
                stale = true;
                break;
            }

            var toAttack = takingTurn.findTargetInRange(cave.entities);
            if (!friendlyMode && toAttack.isPresent()) {
                // Attack
            } else {
                // Move
                var nextMove = takingTurn.tryGetNextMovement(cave);
                if (nextMove.isPresent()) {
                    stale = false;
                    takingTurn.moveTo(nextMove.get());
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
