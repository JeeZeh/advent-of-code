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
        for (int i = 0; i < FlawedFrequencyTransmission.LEN; i++) {
            int phaseIndex = 0;
            int mult = i;
            int sum = 0;
            for (int j = 0; j < FlawedFrequencyTransmission.LEN; j++) {
                if (phaseIndex == 1 || phaseIndex == 3) {
                    int end = Math.min(j + mult, FlawedFrequencyTransmission.LEN) ;
                    int a = (FlawedFrequencyTransmission.sums[end] -
                            FlawedFrequencyTransmission.sums[j]);
                    sum += a * FlawedFrequencyTransmission.PATTERN[phaseIndex];
                }
                phaseIndex = (phaseIndex + 1) % 4;
                j += mult - 1;
                mult = i + 1;
            }
            FlawedFrequencyTransmission.signals[phase + 1][i] = abs(sum) % 10;
        }
//        System.out.println(String.format("BATCH %d - %d COMPLETE", start, end));
    }
}
