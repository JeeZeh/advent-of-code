import java.math.BigInteger;

import java.security.NoSuchAlgorithmException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.*;

public class StockingStuffer {
    private static ExecutorService executorService = Executors.newFixedThreadPool(16);
    private final static int STEP = 1000000;
    private final static int END = 1_000_000_000;
    final static ConcurrentHashMap<String, Integer> map = new ConcurrentHashMap<>();
    final static String key = "yzbqklnj";
    private final static List<Callable<Object>> queue = new ArrayList<>(END / STEP);


    public static void main(String[] args) throws NoSuchAlgorithmException, InterruptedException {

        for (int i = 0; i < END; i += STEP) {
            queue.add(Executors.callable(new HashThread(i, STEP)));
        }

        final long start = System.currentTimeMillis();
        executorService.invokeAll(queue);

        map.forEach((k, v) -> System.out.println(String.format("%s - %d", k, v)));

        final long execTime = System.currentTimeMillis() - start;
        System.out.println(String.format("Batched all Hashes:\nTotal: %d\nMatching: %d\nTime: %ds (%d Hash/ms)",
                END,
                map.size(),
                execTime / 1000,
                END / execTime
        ));
    }

    static String toHex(byte[] bytes) {
        BigInteger bi = new BigInteger(1, bytes);
        return String.format("%0" + (bytes.length << 1) + "X", bi);
    }
}
