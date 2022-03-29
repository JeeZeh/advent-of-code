package aoc;


public class Solution {
    public static void main(String[] args) {
        // Get input
        String input =
                "#########\n#G..G..G#\n#.......#\n#.......#\n#G..E..G#\n#.......#\n#.......#\n#G..G..G#\n#########";
        Game game = new Game(Cave.fromString(input), false);
        partOne(game);
    }

    public static void partOne(Game game) {
        while (game.step().isPresent()) {
            System.out.println(game);
        }
    }
}
