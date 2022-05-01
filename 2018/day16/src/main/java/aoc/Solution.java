package aoc;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.util.HashMap;
import java.util.List;
import java.util.stream.Collectors;

import lombok.Data;

/**
 * Hello world!
 *
 */
public class Solution {
    public static void main(String[] args) throws IOException {
        CPU testCPU = new CPU();

        String input = getResourceFileAsString("aoc/input");
        String[] parts = input.split("\n\n\n");
        String tests = parts[0];
        String program = parts[1];
    }

    HashMap<Integer, String> findInstructions(String tests) {
        String[] lines = tests.split("\n\n");
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
