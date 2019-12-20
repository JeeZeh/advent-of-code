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
    static Map<String, HashMap<String,Dest>> pairs;
    static ExecutorService executorService = Executors.newFixedThreadPool(12);
    static AtomicInteger lowest = new AtomicInteger(100000);
    private static ConcurrentHashMap<State, Integer> bestStates = new ConcurrentHashMap<>();
    static int len;

    public static void main(String[] args) throws IOException {
        String json = Files.readString(Paths.get("./pairs.json"));

        // These are read from JSON. Output from get_pairs() in the python file
        pairs = gson.fromJson(json, new TypeToken<Map<String, HashMap<String, Dest>>>() {
        }.getType());
        len = pairs.size() - 1;
        long s = System.currentTimeMillis();
        getPaths("@", new ArrayList<>(), 0);
        System.out.println(System.currentTimeMillis() - s);
        System.out.println(lowest);
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
            Worlds.lowest.getAndUpdate(i -> Math.min(steps, i));
            return;
        }

        // Get all reachable keys from this key (reachable if I have the keys required to unlock)
        // This is precomputed at runtime for each pair of keys. Paths are the shortest distance between each
        // pair of flags ignoring doors. This assumes no loops.
        Map<String, Dest> potential = Worlds.pairs.get(c).entrySet().stream()
                                                  .filter((e) -> keys.containsAll(e.getValue().req) && !keys
                                                          .contains(e.getKey()))
                                                  .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

        // For every reachable key from the current, explore recursively
        potential.entrySet().parallelStream().forEach(e -> {
            String s = e.getKey();
            Dest v = e.getValue();
            ArrayList<String> keysCopy = (ArrayList<String>) keys.clone();
            keysCopy.add(s);
            getPaths(s, keysCopy, steps + v.dist);
        });

    }


}
