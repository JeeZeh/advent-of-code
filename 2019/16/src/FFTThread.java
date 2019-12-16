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
                if (mult == 0) {
                    phaseIndex++;
                    mult = i + 1;
                } else if (phaseIndex == 0 || phaseIndex == 2) {
                    phaseIndex = (phaseIndex + 1) % 4;
                    j+=mult - 1;
                    mult = i + 1;
                    continue;
                }
                if (phaseIndex == 4) {
                    phaseIndex = 0;
                }
                sum += FlawedFrequencyTransmission.PATTERN[phaseIndex] * FlawedFrequencyTransmission.signals[phase][j];
                mult--;
            }
            FlawedFrequencyTransmission.signals[phase + 1][i] = abs(sum) % 10;
        }
    }
}
