import java.io.IOException;
import java.lang.reflect.Array;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import static java.lang.Math.abs;

public class FlawedFrequencyTransmission {
    final static int[] PATTERN = {0, 1, 0, -1};
    static int LEN = 0;

    public static void main(String[] args) throws IOException {
        List<Integer> base = Arrays.stream(Files.readString(Paths.get("input.txt"))
                .split(""))
                .map(Integer::parseInt)
                .collect(Collectors.toCollection(ArrayList::new));

        int[] input = new int[base.size()*1];

        for (int i = 0; i < base.size()*1; i++) {
            input[i] = base.get(i%base.size());
        }

        LEN = input.length;

        final int MAX = 100;
        final int[][] signals = new int[MAX+1][LEN];

        for (int i = 0; i < base.size(); i++) {
            signals[0][i] = base.get(i);
        }

        long s = System.currentTimeMillis();

        for (int c = 0; c < MAX; c++) {
            for (int i = 0; i < LEN; i++) {
                int phaseIndex = 0;
                int mult = c;
                int sum = 0;
                for (int j = 0; j < LEN; j++) {
                    if (mult == 0) {
                        phaseIndex++;
                        mult = c + 1;
                    }
                    if (phaseIndex == 4) {
                        phaseIndex = 0;
                    }
                    sum += PATTERN[phaseIndex] * signals[c][j];
                    mult--;
                }
                signals[c+1][i] = abs(sum)%10;
            }

        }
        System.out.println(String.format("%dms", System.currentTimeMillis() - s));

        List<Integer> last = new ArrayList<>();
        for (int i = 0; i < signals[MAX].length; i++) {
            last.add(signals[MAX][i]);
        }

        System.out.println(last.stream().map(String::valueOf).collect(Collectors.joining("")).substring(0, 8));
    }

}
