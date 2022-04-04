package aoc;

import java.util.Optional;

public class Game {
    final Cave cave;
    int rounds = 0;
    boolean friendlyMode;
    boolean failWhenElfDies;

    public Game(Cave cave, boolean friendlyMode, boolean failWhenElfDies) {
        this.cave = cave;
        this.friendlyMode = friendlyMode;
        this.failWhenElfDies = failWhenElfDies;
    }

    public void setFriendlyMode(boolean friendlyMode) {
        this.friendlyMode = friendlyMode;
    }

    public Optional<Game> step() throws ElfDiedException {
        this.rounds++;
        boolean incomplete = false;
        
        for (final Entity takingTurn : cave.entities.stream().sorted().toList()) {
            // Since we're looping through a snapshot of the entities as they were
            // at the start of the round, we should check if the entity is dead before
            // giving it a turn
            if (takingTurn.hp <= 0) {
                continue;
            }
            var targets = cave.entities.stream().filter((Entity e) -> takingTurn.isEnemy(e)).toList();
            if (targets.size() == 0) {
                incomplete = true;
                break;
            }

            // Try attack
            var toAttack = takingTurn.findTargetInRange(cave.entities);
            if (toAttack.isEmpty()) {
                // Move
                var nextMove = takingTurn.tryGetNextMovement(cave);
                if (nextMove.isPresent()) {
                    takingTurn.moveTo(nextMove.get().firstMove.get());
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
                    if (failWhenElfDies && target.type == EntityType.Elf) {
                        throw new ElfDiedException();
                    }
                }
                // System.out.println("%s attacks %s".formatted(takingTurn.asDebug(), target.asDebug()));
            }
        }

        // This is the end state, if we ever find that we can't complete a round (any move, even the first)
        // then we end the round and count it as incomplete
        if (incomplete) {
            // Only count full rounds
            this.rounds--;
            return Optional.empty();
        }
        return Optional.of(this);
    }

    public Game play(boolean debug) throws ElfDiedException {
        while (this.step().isPresent()) {
            if (debug) {
                System.out.println(this);
                System.out.println();
            }
        }
        if (debug) {
            System.out.println(this);
            System.out.println();
        }
        return this;
    }

    public GameSummary getSummary() {
        return new GameSummary(this);
    }

    @Override
    public String toString() {
        return cave.toString();
    }
}

class ElfDiedException extends Exception {

}
