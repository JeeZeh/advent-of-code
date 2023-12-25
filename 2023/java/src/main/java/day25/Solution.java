package day25;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.Objects;
import java.util.Set;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;
import lib.Input;

public class Solution {

  public static class Node {

    final String id;
    final Set<Node> connected = new HashSet<>();

    public Node(String id) {
      this.id = id;
    }

    public void addConnected(Node node) {
      connected.add(node);
    }

    @Override
    public String toString() {
      return STR."\{id} (\{connected.stream().map(n -> n.id).collect(Collectors.joining(", "))})";
    }
  }

  static List<Node> parseNodes(List<String> lines) {
    Map<Node, List<String>> nodeConnections = new HashMap<>();
    List<Node> nodes = new ArrayList<>();
    for (String line : lines) {
      var parts = line.split(": ");
      var src = parts[0];
      var out = parts[1].split(" ");
      var maybeSrcNode = nodes.stream().filter(n -> n.id.equals(src)).findFirst();
      Node srcNode;
      if (maybeSrcNode.isPresent()) {
        srcNode = maybeSrcNode.get();
      } else {
        srcNode = new Node(src);
        nodes.add(srcNode);
      }

      for (String connection : out) {
        if (nodes.stream().noneMatch(n -> n.id.equals(connection))) {
          nodes.add(new Node(connection));
        }
      }
      nodeConnections.put(srcNode, new ArrayList<>(Arrays.stream(out).toList()));
    }

    nodeConnections.forEach((src, out) -> {
      out.stream().map(id -> nodes.stream().filter(n -> n.id.equals(id)).findFirst().get())
          .forEach(o -> {
            src.addConnected(o);
            o.addConnected(src);
          });
    });

    return nodes;
  }

  public record Step(Node node, List<Pair> path) {

  }

  static List<Pair> shortestPath(Node from, Node to) {
    Set<Node> seen = new HashSet<>();
    Deque<Step> nextNodes = new ArrayDeque<>();

    nextNodes.add(new Step(from, new ArrayList<>()));
    seen.add(from);

    while (!nextNodes.isEmpty()) {
      var current = nextNodes.poll();

      if (current.node.equals(to)) {
        return current.path;
      }

      current.node.connected.stream().filter(c -> !seen.contains(c)).forEach(n -> {
        var copy = new ArrayList<>(current.path);
        copy.add(new Pair(current.node, n));
        nextNodes.add(new Step(n, copy));
        seen.add(n);
      });
    }

    return new ArrayList<>();
  }


  static Set<Node> fill(Node start, Map<Node, Node> illegalWalks) {
    Set<Node> seen = new HashSet<>();
    Deque<Node> nextNodes = new ArrayDeque<>();

    nextNodes.add(start);
    seen.add(start);

    while (!nextNodes.isEmpty()) {
      var current = nextNodes.poll();

      current.connected.stream().filter(c -> !seen.contains(c)).forEach(n -> {

        var illegalWalk = illegalWalks.get(current);
        if (illegalWalk == null || illegalWalk != n) {
          nextNodes.add(n);
          seen.add(n);
        }
      });
    }

    return seen;
  }


  public record Pair(Node a, Node b) {

    @Override
    public int hashCode() {
      return Stream.of(a, b).map(n -> n.id).sorted().collect(Collectors.joining(",")).hashCode();
    }
  }

  static long flakyMinCut(List<Node> nodes, int iterations) {
    // Keep retrying this algorithm until we get a non-zero answer.
    while (true) {
      // Randomly pick nodes that are not connected directly, up to iteration count, and find the
      // shortest path between them. Then pick the top 3. This is not guaranteed to be the best cut.
      var topThree = IntStream.range(0, iterations).parallel().mapToObj(x -> {
            var first = nodes.get((int) (Math.random() * nodes.size()));
            var second = nodes.get((int) (Math.random() * nodes.size()));
            if (first.connected.contains(second)) {
              return null;
            }
            return shortestPath(first, second).stream();
          }).filter(Objects::nonNull).flatMap(Function.identity())
          .collect(Collectors.groupingBy(Function.identity(), Collectors.counting())).entrySet()
          .stream().sorted((a, b) -> -a.getValue().compareTo(b.getValue())).limit(4).toList();

      // Disallow walking these paths (pretend cut)
      Map<Node, Node> illegalWalks = new HashMap<>();
      topThree.stream().map(Entry::getKey).forEach(pair -> {
        var a = nodes.stream().filter(n -> pair.a.id.equals(n.id)).findAny().get();
        var b = nodes.stream().filter(n -> pair.b.id.equals(n.id)).findAny().get();
        illegalWalks.put(a, b);
        illegalWalks.put(b, a);
      });

      // Fill from one side of the cut and find the size of each resulting subgraph
      var fillA = fill(nodes.stream().filter(n -> !illegalWalks.containsKey(n)).findFirst().get(),
          illegalWalks);
      var result = (long) (nodes.size() - fillA.size()) * fillA.size();
      if (result != 0) {
        return result;
      }
    }
  }

  public static void main(String[] args) throws IOException {
    List<Node> nodes = parseNodes(Input.lines("day25/input.txt").toList());
    long partOne = flakyMinCut(nodes, 100);
    String partTwo = "Merry Christmas";
    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }
}
