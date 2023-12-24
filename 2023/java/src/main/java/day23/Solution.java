package day23;

import java.io.IOException;
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Set;
import lib.Grid;
import lib.Input;
import lib.Pos;
import lib.Pos.Direction;

public class Solution {

  private static final List<Direction> DIRECTIONS = List.of(Direction.UP, Direction.RIGHT,
      Direction.DOWN, Direction.LEFT);

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day23/input.txt").toList();
    var trail = Trail.fromLines(lines);
    trail.asGraph(false);

    long partOne = trail.asGraph(false).longestWalk();
    System.out.println(STR."Part 1: \{partOne}");
    long partTwo = trail.asGraph(true).longestWalk();
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public record Trail(List<List<Tile>> elements) implements Grid<Tile> {

    record Step(Pos pos, boolean[][] walked, long total, List<Pos> section, Set<Pos> shortcuts) {

      boolean[][] copyWalkedWith(int x, int y) {
        boolean[][] walked = new boolean[this.walked.length][];
        for (int i = 0; i < this.walked.length; i++) {
          walked[i] = this.walked[i].clone();
        }
        walked[y][x] = true;
        return walked;
      }
    }

    List<Step> getNextSteps(Step curr, boolean allowSlope) {
      // Must slide?
      var tile = get(curr.pos);
      if (tile != Tile.PATH) {
        var heading = switch (get(curr.pos)) {
          case SLOPE_UP -> Direction.UP;
          case SLOPE_RIGHT -> Direction.RIGHT;
          case SLOPE_DOWN -> Direction.DOWN;
          case SLOPE_LEFT -> Direction.LEFT;
          default -> throw new IllegalStateException(STR."Unexpected value: \{get(curr.pos)}");
        };
        Pos walk = heading.transpose(curr.pos);
        if (!curr.walked[walk.y()][walk.x()]) {
          var newWalked = curr.copyWalkedWith(walk.x(), walk.y());
          var copySection = new ArrayList<>(curr.section);
          copySection.add(walk);
          return List.of(new Step(walk, newWalked, curr.total + 1, copySection, new HashSet<>()));
        }
        if (!allowSlope) {
          return List.of();
        }
      }

      // Move in all directions
      return DIRECTIONS.stream().map(dir -> {
        Pos newPos = dir.transpose(curr.pos);
        if (isWithin(newPos)) {
          var newTile = get(newPos);
          if (newTile != Tile.FOREST) {
            if (!curr.walked[newPos.y()][newPos.x()]) {
              var newWalked = curr.copyWalkedWith(newPos.x(), newPos.y());
              var copySection = new ArrayList<>(curr.section);
              copySection.add(newPos);
              return new Step(newPos, newWalked, curr.total + 1, copySection, new HashSet<>());
            }
          }
        }
        return null;
      }).filter(Objects::nonNull).toList();
    }

    List<List<Pos>> nextJunctions(Pos from, Set<Pos> seenJunctionEntrances,
        Set<Pos> newJunctionEntrances, boolean allowSlope) {
      List<List<Pos>> junctions = new ArrayList<>();
      Pos goal = new Pos(this.width() - 2, this.height() - 1);
      var initialWalked = new boolean[width()][height()];
      initialWalked[from.y()][from.x()] = true;
      var initial = new Step(from, initialWalked, 0, new ArrayList<>(List.of(from)),
          new HashSet<>());
      Deque<Step> nextSteps = new ArrayDeque<>(List.of(initial));

      while (!nextSteps.isEmpty()) {
        var curr = nextSteps.poll();

        var nexts = getNextSteps(curr, allowSlope);

        // At junction
        if (!curr.pos.equals(from) && (nexts.size() > 1 || curr.pos.equals(goal))) {
          junctions.add(new ArrayList<>(curr.section));
          newJunctionEntrances.add(curr.section().get(curr.section.size() - 2));
        } else {
          nexts.stream().filter(next -> !seenJunctionEntrances.contains(next.pos))
              .forEach(nextSteps::add);
        }
      }

      return junctions;
    }

    Map<Pos, Set<List<Pos>>> buildShortcuts(boolean allowSlope) {
      Map<Pos, Set<List<Pos>>> shortcuts = new HashMap<>();

      Pos start = new Pos(1, 0);
      Deque<Pos> nextJunctions = new ArrayDeque<>(List.of(start));
      Set<Pos> seenJunctionEntrances = new HashSet<>();
      Set<Pos> newJunctionEntrances = new HashSet<>();

      while (!nextJunctions.isEmpty()) {
        var junction = nextJunctions.poll();

        var connecting = shortcuts.computeIfAbsent(junction, k -> new HashSet<>());
        var nexts = nextJunctions(junction, seenJunctionEntrances, newJunctionEntrances,
            allowSlope);
        seenJunctionEntrances = new HashSet<>(newJunctionEntrances);
        newJunctionEntrances.clear();
        connecting.addAll(nexts);
        nexts.stream().map(List::getLast).filter(l -> !shortcuts.containsKey(l))
            .forEach(nextJunctions::add);
      }

      return shortcuts;
    }

    Graph asGraph(boolean allowSlope) {
      Pos start = new Pos(1, 0);
      Pos goal = new Pos(this.width() - 2, this.height() - 1);

      List<Node> nodes = new ArrayList<>();
      List<Edge> edges = new ArrayList<>();

      var cuts = buildShortcuts(allowSlope);

      cuts.keySet().stream().map(Node::new).forEach(nodes::add);
      cuts.forEach((key, value) -> {
        var from = nodes.stream().filter(n -> n.pos.equals(key)).findAny().get();
        value.forEach(cut -> {
          var to = nodes.stream().filter(n -> n.pos.equals(cut.getLast())).findAny().get();
          var edge = new Edge(from, to, cut.size() - 1);
          from.addEdge(edge);
          edges.add(edge);
        });
      });

      return new Graph(nodes.stream().filter(n -> n.pos.equals(start)).findAny().get(),
          nodes.stream().filter(n -> n.pos.equals(goal)).findAny().get(), nodes, edges);
    }

    static Trail fromLines(List<String> lines) {
      return new Trail(
          lines.stream().map(line -> line.chars().mapToObj(c -> Tile.fromChar((char) c)).toList())
              .toList());
    }
  }


  public static class Node {

    final Pos pos;
    final List<Edge> outgoing;

    public Node(Pos pos) {
      this.pos = pos;
      this.outgoing = new ArrayList<>();
    }

    public void addEdge(Edge e) {
      this.outgoing.add(e);
    }
  }

  public record Edge(Node from, Node to, long weight) {

  }

  public record Graph(Node start, Node end, List<Node> nodes, List<Edge> edges) {

    record GraphStep(Node node, long dist, List<Pos> walked) {

    }

    public long longestWalk() {
      // Needs some optimisation like A* or something (not sure how to prune tho)
      Deque<GraphStep> nextSteps = new ArrayDeque<>();
//          Comparator.comparing(s -> -s.seen.size()));

      nextSteps.add(new GraphStep(start, 0, new ArrayList<>(List.of(start.pos))));

      long highest = Long.MIN_VALUE;
      while (!nextSteps.isEmpty()) {
        var curr = nextSteps.pop();
        if (curr.node.equals(end)) {
          highest = Math.max(curr.dist, highest);
        } else {
          curr.node.outgoing.forEach(edge -> {
            if (!curr.walked().contains(edge.to.pos)) {
              var newWalked = new ArrayList<>(curr.walked);
              newWalked.add(edge.to.pos);
              nextSteps.add(new GraphStep(edge.to, curr.dist + edge.weight, newWalked));
            }
          });
        }
      }
      return highest;
    }
  }

  public enum Tile {
    PATH, FOREST, SLOPE_UP, SLOPE_RIGHT, SLOPE_DOWN, SLOPE_LEFT;

    static Tile fromChar(char c) {
      return switch (c) {
        case '.' -> PATH;
        case '#' -> FOREST;
        case '^' -> SLOPE_UP;
        case '>' -> SLOPE_RIGHT;
        case 'v' -> SLOPE_DOWN;
        case '<' -> SLOPE_LEFT;
        default -> throw new IllegalStateException(STR."Unexpected value: \{c}");
      };
    }

    @Override
    public String toString() {
      return switch (this) {
        case PATH -> ".";
        case FOREST -> "#";
        case SLOPE_UP -> "^";
        case SLOPE_RIGHT -> ">";
        case SLOPE_DOWN -> "v";
        case SLOPE_LEFT -> "<";
      };
    }
  }
}
