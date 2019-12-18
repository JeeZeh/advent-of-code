import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.Executor;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;

@SuppressWarnings("unchecked")
public class Worlds {
    static Gson gson = new GsonBuilder().create();
    static Map<String, HashMap<String,Dest>> pairs;
    static ExecutorService executorService = Executors.newFixedThreadPool(12);
    static AtomicInteger lowest = new AtomicInteger(100000);
    static AtomicInteger perms = new AtomicInteger(0);
    static int len;

    public static void main(String[] args) throws IOException {
        String json = Files.readString(Paths.get("./pairs.json"));
        System.out.println(json);
        pairs = gson.fromJson(json, new TypeToken<Map<String, HashMap<String,Dest>>>() {}.getType());
        System.out.println(pairs.get("@").get("g").dist);
        len = pairs.size()-1;
        getPaths("@", new ArrayList<String>(), 0);
        System.out.println(len);
        System.out.println(lowest);


//        Map<String, Dest> starters = pairs.get("@").entrySet().stream()
//                                           .filter((e) -> e.getValue().req.size() == 0)
//                                           .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
//
//        for (Map.Entry<String, Dest> entry : starters.entrySet()) {
//            String s = entry.getKey();
//            Dest v = entry.getValue();
//
//            executorService.submit(new ThreadedPath(s, new ArrayList<>(Collections.singletonList(s)), v.dist));
//        }

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
