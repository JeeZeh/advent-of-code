package aoc;

enum Tile {
    Wall("#"),
    Floor(".");

    public final String label;

    private Tile(String label) {
        this.label = label;
    }
}
