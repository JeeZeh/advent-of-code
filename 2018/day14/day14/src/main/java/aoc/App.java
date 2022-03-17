package aoc;

public class App {
    public static void main(String[] args) {
        final int partOne = 503761;
        final String partTwo = "503761";
        final StringBuilder scores = new StringBuilder("37");
        int e1 = 0;
        int e2 = 1;
        int foundAt = -1;
        while (foundAt == -1) {
            int newScore = (scores.charAt(e1) - '0') + (scores.charAt(e2) - '0');
            scores.append(newScore);

            e1 = (e1 + 1 + (scores.charAt(e1) - '0')) % scores.length();
            e2 = (e2 + 1 + (scores.charAt(e2) - '0')) % scores.length();

            foundAt = scores.substring(Math.max(0, scores.length() - partTwo.length() - 1)).indexOf(partTwo);
        }
        System.out.println(String.format("Part 1: %s", scores.substring(partOne, partOne + 10)));
        System.out.println(String.format("Part 2: %d", scores.length() - partTwo.length() - (1 - foundAt)));
    }
}
