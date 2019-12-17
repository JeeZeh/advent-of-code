import java.util.stream.IntStream;

import static java.lang.Math.abs;

public class FFTThread extends Thread {

    private int start;
    private int end;
    private int phase;

    FFTThread(int start, int end, int phase) {
        this.start = start;
        this.end = end;
        this.phase = phase;
    }

    @Override
    public void run() {
        for (int i = start; i < end; i++) {
            int phaseIndex = 0;
            int mult = i;
            int sum = 0;
            for (int j = 0; j < FlawedFrequencyTransmission.LEN; j++) {
                if (phaseIndex == 1 || phaseIndex == 3) {
                    int end = Math.min(j + mult, FlawedFrequencyTransmission.LEN);
                    int ssum = java.util.Arrays.stream(FlawedFrequencyTransmission.signals[phase], j, end)
                            .sum();
                    sum +=  ssum * FlawedFrequencyTransmission.PATTERN[phaseIndex];
                }
                phaseIndex = (phaseIndex + 1) % 4;
                j+=mult -1;
                mult = i + 1;
            }
            FlawedFrequencyTransmission.signals[phase + 1][i] = abs(sum) % 10;
        }
    }
}
