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
<<<<<<< HEAD
    static AtomicInteger perms = new AtomicInteger(0);
    static ConcurrentHashMap<State, Integer> bestStates = new ConcurrentHashMap<>();
    static ConcurrentHashMap<Integer, List<String>> bestStrings = new ConcurrentHashMap<>();
=======
    private static ConcurrentHashMap<State, Integer> bestStates = new ConcurrentHashMap<>();
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d
    static int len;

    public static void main(String[] args) throws IOException {
        String json = Files.readString(Paths.get("./pairs.json"));
<<<<<<< HEAD
        System.out.println(json);
        pairs = gson.fromJson(json, new TypeToken<Map<String, HashMap<String,Dest>>>() {}.getType());
        System.out.println(pairs.get("@").get("g").dist);
        len = pairs.size()-1;
        getPaths("@", new ArrayList<String>(), 0);
        System.out.println(len);
        System.out.println(lowest);
        System.out.println(perms.get());

        List sortedKeys=new ArrayList(bestStrings.keySet());
        Collections.sort(sortedKeys);
        sortedKeys.forEach(k -> {
            System.out.println();
            System.out.print(k);
            System.out.print(" ");
            System.out.print(bestStrings.get(k));
        });
=======

        // These are read from JSON. Output from get_pairs() in the python file
        pairs = gson.fromJson(json, new TypeToken<Map<String, HashMap<String, Dest>>>() {
        }.getType());
        len = pairs.size() - 1;
        long s = System.currentTimeMillis();
        getPaths("@", new ArrayList<>(), 0);
        System.out.println(System.currentTimeMillis() - s);
        System.out.println(lowest);
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d
    }


    private static void getPaths(String c, ArrayList<String> keys, int steps) {
        // Hash the current state
        // State is the current position (current key being picked up) and set of keys collected upto this point
        final State currentState = new State(c, keys);
        if (bestStates.containsKey(currentState)) {
<<<<<<< HEAD
=======
            // If the current states matches a previous state with fewer steps
            // then cancel recursion as we have a better version in the past - I think
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d
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

<<<<<<< HEAD
        if (keys.size() == Worlds.len) {
            Worlds.perms.incrementAndGet();
            Worlds.lowest.getAndUpdate(i -> steps < i ? steps : i );
            bestStrings.put(steps, keys);
            if (Worlds.perms.get() % 100000 == 0) {
                System.out.println(String.format("Permutations: %d+\nShortest: %d steps", Worlds.perms.get(), Worlds.lowest.get()));
            }
            return;
        }

=======
        // For every reachable key from the current, explore recursively
>>>>>>> 5d3adb0b40b55289bf3af7b5a295ccb0c02e3f6d
        potential.entrySet().parallelStream().forEach(e -> {
            String s = e.getKey();
            Dest v = e.getValue();
            ArrayList<String> keysCopy = (ArrayList<String>) keys.clone();
            keysCopy.add(s);
            getPaths(s, keysCopy, steps + v.dist);
        });

    }


}
