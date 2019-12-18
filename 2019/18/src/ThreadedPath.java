import java.util.ArrayList;
import java.util.Map;
import java.util.stream.Collectors;

public class ThreadedPath extends Thread {
    private String initialChars;
    private ArrayList<String> initialKeys;
    private int initialSteps;

    public ThreadedPath(String initialChars, ArrayList<String> initialKeys, int initialSteps) {
        this.initialChars = initialChars;
        this.initialKeys = initialKeys;
        this.initialSteps = initialSteps;
    }

    @Override
    public void run() {
        getPaths(initialChars, initialKeys, initialSteps);
        System.out.println("----------THREAD CLOSED----------");
        System.out.println("----------THREAD CLOSED----------");
        System.out.println("----------THREAD CLOSED----------");
        System.out.println("----------THREAD CLOSED----------");
        System.out.println("----------THREAD CLOSED----------");
    }

    public static void getPaths(String c, ArrayList<String> keys, int steps) {
        Map<String, Dest> potential = Worlds.pairs.get(c).entrySet().stream()
                                           .filter((e) -> keys.containsAll(e.getValue().req) && !keys
                                                   .contains(e.getKey()))
                                           .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

        if (keys.size() == Worlds.len) {
            Worlds.perms.incrementAndGet();
            Worlds.lowest.getAndUpdate(i -> steps < i ? steps : i );
            if (Worlds.perms.get() % 100000 == 0) {
                System.out.println(String.format("Permutations: %d+\nShortest: %d steps", Worlds.perms.get(), Worlds.lowest.get()));
            }
            return;
        }

        potential.entrySet().parallelStream().forEach(e -> {
            String s = e.getKey();
            Dest v = e.getValue();
            ArrayList<String> keysCopy = (ArrayList<String>) keys.clone();
            keysCopy.add(s);
            getPaths(s, keysCopy, steps + v.dist);
        });

    }
}
