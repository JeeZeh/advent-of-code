package aoc;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.stream.Collectors;

public class Solution {
    public static void main(String[] args) throws IOException, ElfDiedException {
        // Get input
        String input = getResourceFileAsString("aoc/input.txt");

        partOne(input);
        partTwo(input);
    }

    public static void partOne(String input) throws ElfDiedException {
        Game game = new Game(Cave.fromString(input), false, false);
        game.play(false);
        System.out.println(game.getSummary());
    }

    public static void partTwo(String input) {
        // Override Elf config
        Game game = null;
        
        System.out.println(game.getSummary());
    }

    /**
     * Reads given resource file as a string.
     *
     * @param fileName path to the resource file
     * @return the file's contents
     * @throws IOException if read fails for any reason
     */
    static String getResourceFileAsString(String fileName) throws IOException {
        ClassLoader classLoader = ClassLoader.getSystemClassLoader();
        try (InputStream is = classLoader.getResourceAsStream(fileName)) {
            if (is == null)
                return null;
            try (InputStreamReader isr = new InputStreamReader(is, StandardCharsets.UTF_8);
                    BufferedReader reader = new BufferedReader(isr)) {
                return reader.lines().collect(Collectors.joining(System.lineSeparator()));
            }
        }
    }
}
