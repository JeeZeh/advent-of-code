import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executor;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;

@SuppressWarnings("unchecked")
public class Worlds {
    private static Gson gson = new GsonBuilder().create();
    static Map<String, List<Dest>> pairs;
    static ExecutorService executorService = Executors.newFixedThreadPool(12);
    static AtomicInteger lowest = new AtomicInteger(100000);
    static AtomicInteger perms = new AtomicInteger(0);
    private static ConcurrentHashMap<State, Integer> bestStates = new ConcurrentHashMap<>();
    private static ConcurrentHashMap<Integer, ArrayList<String>> bestPaths = new ConcurrentHashMap<>();
    static int len;

    public static void main(String[] args) throws IOException {
        String json = Files.readString(Paths.get("./pairs.json"));

        // These are read from JSON. Output from get_pairs() in the python file
        pairs = gson.fromJson(json, new TypeToken<Map<String, List<Dest>>>() {}.getType());
        len = pairs.size()-1;
        getPaths("@", new ArrayList<>(), 0);
        System.out.println(len);
        System.out.println(lowest);
        List sortedKeys = new ArrayList(bestPaths.keySet());
        Collections.sort(sortedKeys);

        sortedKeys.forEach(e -> {
            System.out.println();
            System.out.print(e);
            System.out.print(" ");
            System.out.print(bestPaths.get(e));
        });
    }

    private static void getPaths(String c, ArrayList<String> keys, int steps) {
        // Hash the current state
        // State is the current position (current key being picked up) and set of keys collected upto this point
        final State currentState = new State(c, keys);
        if (bestStates.containsKey(currentState)) {
            // If the current states matches a previous state with fewer steps
            // then cancel recursion as we have a better version in the past - I think
            if (bestStates.get(currentState) <= steps) {
                return;
            }
        }

        // Store the current state with current steps
        bestStates.put(currentState, steps);

        // If we have all the keys, store the path and lengths, incr. permutations
        if (keys.size() == Worlds.len) {
            bestPaths.put(steps, keys);
            Worlds.perms.incrementAndGet();
            Worlds.lowest.getAndUpdate(i -> Math.min(steps, i));
            if (Worlds.perms.get() % 100000 == 0) {
                System.out.println(String.format("Permutations: %d+\nShortest: %d steps", Worlds.perms.get(), Worlds.lowest.get()));
            }
            return;
        }

        // Get all reachable keys from this key (reachable if I have the keys required to unlock)
        // This is precomputed at runtime for each pair of keys. Paths are the shortest distance between each
        // pair of flags ignoring doors. This assumes no loops.
        List<Dest> potential = Worlds.pairs.get(c).stream()
                                                  .filter((e) -> keys.containsAll(e.req) && !keys
                                                          .contains(e.key))
                                                  .collect(Collectors.toList());

        // For every reachable key from the current, explore recursively
        potential.parallelStream().forEach(e -> {
            ArrayList<String> keysCopy = (ArrayList<String>) keys.clone();
            keysCopy.add( e.key);
            getPaths(e.key, keysCopy, steps + e.dist);
        });

    }


}
