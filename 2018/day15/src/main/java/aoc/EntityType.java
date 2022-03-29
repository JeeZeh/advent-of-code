package aoc;

public enum EntityType {
    Elf("E"),
    Goblin("G");

    public final String label;

    private EntityType(String label) {
        this.label = label;
    }
}
