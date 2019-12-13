import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class HashThread extends Thread {
    private MessageDigest md;
    private int start;
    private int end;

    HashThread(int start, int size) throws NoSuchAlgorithmException {
        this.md = MessageDigest.getInstance("MD5");
        this.start = start;
        this.end = start + size;
    }


    @Override
    public void run() {
        System.out.println(String.format("Batching %d-%d", start, end));
        for (int i = start; i < end; i++) {
            byte[] h = md.digest(String.format("%s%05d", StockingStuffer.key, i).getBytes(StandardCharsets.UTF_8));
            String hash = StockingStuffer.toHex(h);
            if (hash.startsWith("000000")) {
                StockingStuffer.map.put(hash, i);
            }
        }
    }
}
