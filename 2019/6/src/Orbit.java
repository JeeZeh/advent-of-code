import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.stream.Collectors;

public class Orbit {
    final static Map<String, String> orbits = new HashMap<>();
    final static Set<String> explored = new HashSet<>();
    final static long INIT = System.currentTimeMillis();

    public static void main(String[] args) {

        final String fileName = "./input.txt";
        final List<String> lines = new ArrayList<>();


        try {
            final Path path = Paths.get(fileName);
            lines.addAll(Files.readAllLines(path, StandardCharsets.UTF_8));
        } catch (IOException ioe) {
            System.err.println(ioe.toString());
        }

        for (String line: lines) {
            orbits.put(line.split("\\)")[1], line.split("\\)")[0]);
        }

        int count = 0;
        for (final String v : orbits.values()){
            String o = v;
            count++;
            while (!o.equals("COM")) {
                o = orbits.get(o);
                count++;
            }
        }

        System.out.println(count);

        recurse("YOU", 0);
    }

    public static List<String> getSurrounding(String node) {
        // Get everything that orbits node
        List<String> next = orbits.entrySet()
                .stream()
                .filter(e -> node.equals(e.getValue()))
                .map(Map.Entry::getKey)
                .collect(Collectors.toList());

        // Add what node orbits
        next.add(orbits.get(node));
        return next;
    }

    public static void recurse(String s,int steps) {
        if (s.equals("SAN")) {
            System.out.println(steps);
            System.out.println(System.currentTimeMillis() - INIT);
            System.exit(0);
        } else {
            explored.add(s);

            List<String> filtered = getSurrounding(s)
                    .stream()
                    .filter(e -> !explored.contains(e))
                    .collect(Collectors.toList());

            for (String node : filtered) {
                recurse(node, steps + 1);
            }
        }
    }
}
