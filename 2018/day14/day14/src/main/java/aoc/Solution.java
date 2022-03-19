package aoc;

public class Solution {
    int recipeCount;
    String recipeCountString;
    StringBuilder recipes;

    public Solution(String recipeCountString) {
        this.recipeCount = Integer.parseInt(recipeCountString);
        this.recipeCountString = recipeCountString;
        recipes = this.generateRecipes();
    }

    StringBuilder generateRecipes() {
        final StringBuilder scores = new StringBuilder("37");
        int e1 = 0;
        int e2 = 1;
        int foundAt = -1;
        while (foundAt == -1 || scores.length() < this.recipeCount + 10) {
            int scoreOne = (scores.charAt(e1) - '0');
            int scoreTwo = (scores.charAt(e2) - '0');
            scores.append(scoreOne + scoreTwo);
            e1 = (e1 + 1 + (scoreOne)) % scores.length();
            e2 = (e2 + 1 + (scoreTwo)) % scores.length();
            foundAt = scores.substring(Math.max(0, scores.length() - this.recipeCountString.length() - 1))
                    .indexOf(recipeCountString);
        }

        return scores;
    }

    String partOne() {
        return this.recipes.substring(this.recipeCount, this.recipeCount + 10);
    }

    String partTwo() {
        return String.format("%d", this.recipes.indexOf(recipeCountString));
    }
}

class Main {
    public static void main(String[] args) {
        final Solution solution = new Solution("503761");
        System.out.println(String.format("Part 1: %s", solution.partOne()));
        System.out.println(String.format("Part 2: %s", solution.partTwo()));
    }
}