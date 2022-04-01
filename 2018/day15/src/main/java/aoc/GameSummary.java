package aoc;

import java.util.stream.Collectors;

public class GameSummary {
    final int rounds;
    final int totalHpRemaining;
    final int outcome;
    final EntityType winner;

    public GameSummary(Game game) {
        rounds = game.rounds;
        totalHpRemaining = game.cave.entities
                .stream()
                .map((Entity e) -> e.hp)
                .collect(Collectors.summingInt(Integer::intValue));
        outcome = rounds * totalHpRemaining;
        winner = game.cave.entities.get(0).type;
    }

    @Override
    public String toString() {
        String winnerType = winner == EntityType.Elf ? "Elves" : "Goblins";
        StringBuilder sb = new StringBuilder();
        sb.append("Combat ends after %d full rounds\n".formatted(rounds));
        sb.append("%s win with %d total hit points left\n".formatted(winnerType, totalHpRemaining));
        sb.append("Outcome: %d * %d = %d".formatted(rounds, totalHpRemaining, outcome));
        return sb.toString();
    }
}
