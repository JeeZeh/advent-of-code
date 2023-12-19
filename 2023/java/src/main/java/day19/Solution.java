package day19;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import lib.Input;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.MutablePair;
import org.apache.commons.lang3.tuple.Pair;

public class Solution {

  final static int REJECTED = -1;
  final static int ACCEPTED = -2;
  final static int NEXT = -3;

  public static void main(String[] args) throws IOException {
    var inputParts = Input.read("day19/input.txt").split("\r\n\r\n");
    var workflowIds = inputParts[0].lines().filter(l -> !l.isEmpty()).map(l -> l.split("\\{")[0])
        .toList();
    var workflows = inputParts[0].lines().filter(l -> !l.isEmpty())
        .map(l -> Workflow.fromLine(l, workflowIds)).toList();
    var parts = inputParts[1].lines().map(Solution::parsePart).toList();

    long partOne = parts.stream().filter(p -> trial(p, workflows, workflowIds.indexOf("in")))
        .mapToLong(p -> Arrays.stream(p).sum()).sum();
    long partTwo = getTotalCombinations(workflows, workflowIds.indexOf("in"));

    System.out.println(STR."Part 1: \{partOne}");
    System.out.println(STR."Part 2: \{partTwo}");
  }

  public static long getTotalCombinations(List<Workflow> workflows, int inId) {
    long total = 0;
    for (int workflow = 0; workflow < workflows.size(); workflow++) {
      for (int rule = 0; rule < workflows.get(workflow).rules.size(); rule++) {
        if (workflows.get(workflow).rules.get(rule).destination == ACCEPTED) {
          total += getCombinationsForWorkflow(new ImmutablePair<>(workflows.get(workflow), rule),
              workflows, inId, new MutablePair[]{new MutablePair(1, 4000), new MutablePair(1, 4000),
                  new MutablePair(1, 4000), new MutablePair(1, 4000)});
        }
      }
    }

    return total;
  }

  public static long getCombinationsForWorkflow(Pair<Workflow, Integer> starting,
      List<Workflow> workflows, int inId, MutablePair<Integer, Integer>[] partLimits) {
    var currentWorkflow = starting.getLeft();
    for (int currentRule = starting.getRight(); currentRule >= 0; currentRule--) {
      // Move the limit based on which rules we need to accept and break
      var rule = currentWorkflow.rules.get(currentRule);

      // 1. Constrain TO the rule which sends us to the last workflow (or accepts us if it's the start)
      if (currentRule == starting.getRight()) {
        if (rule.condition == '>') {
          // We want to be above this value to meet
          if (partLimits[rule.check].left <= rule.value) {
            partLimits[rule.check].setLeft(rule.value + 1);
          }
        } else if (rule.condition == '<') {
          // We want to be below this value to meet
          if (partLimits[rule.check].right >= rule.value) {
            partLimits[rule.check].setRight(rule.value - 1);
          }
        }
        continue;
      }

      // 2. BREAK every rule to the left of it
      if (rule.condition == '>') {
        // We want to be at or below this value to break
        if (partLimits[rule.check].right > rule.value) {
          partLimits[rule.check].setRight(rule.value);
        }
      } else if (rule.condition == '<') {
        // We want to be at or above this value to break
        if (partLimits[rule.check].left < rule.value) {
          partLimits[rule.check].setLeft(rule.value);
        }
      }
      if (Arrays.stream(partLimits).anyMatch(pair -> pair.left > pair.right)) {
        return 0;
      }
    }

    // Find what sends us to the current workflow
    //    - Workflows seem to only be mapped 1:M, so we only need to walk from A -> in
    if (currentWorkflow.id != inId) {
      var nextWorkflow = workflows.stream().filter(workflow -> workflow.rules.stream()
          .anyMatch(rule -> rule.destination == currentWorkflow.id)).findFirst().get();
      var nextRule = nextWorkflow.rules.stream()
          .filter(rule -> rule.destination == currentWorkflow.id).findFirst();
      var nextRuleId = nextWorkflow.rules.indexOf(nextRule.get());
      return getCombinationsForWorkflow(new ImmutablePair<>(nextWorkflow, nextRuleId), workflows,
          inId, partLimits);
    }

    // 172800000000000
    // 167409079868000
    return Arrays.stream(partLimits).mapToLong(pair -> pair.right - pair.left + 1)
        .reduce((a, acc) -> a * acc).stream().sum();
  }

  public static boolean trial(long[] part, List<Workflow> workflows, int startingWorkflowId) {
    Workflow currentWorkflow = workflows.get(startingWorkflowId);
    while (true) {
      for (var rule : currentWorkflow.rules) {
        var destination = rule.apply(part);
        if (destination != NEXT) {
          if (destination == ACCEPTED) {
            return true;
          } else if (destination == REJECTED) {
            return false;
          } else {
            currentWorkflow = workflows.get(destination);
            break;
          }
        }
      }
    }

//    throw new IllegalStateException(STR."No trial result for part: \{Arrays.toString(part)}");
  }

  public static long[] parsePart(String line) {
    return Arrays.stream(line.substring(1, line.length() - 1).split(",")).map(p -> p.split("=")[1])
        .mapToLong(Long::parseLong).toArray();
  }

  public record Workflow(int id, List<Rule> rules) {

    static Workflow fromLine(String line, List<String> workflowIds) {
      var parts = line.split("\\{");
      var id = workflowIds.indexOf(parts[0]);
      List<Rule> rules = new ArrayList<>();
      var ruleStrings = parts[1].substring(0, parts[1].length() - 1).split(",");

      for (String rule : ruleStrings) {
        var ruleParts = rule.split(":");
        if (ruleParts.length == 1) {
          rules.add(new Rule('-', 0, 0, workflowIdFromString(ruleParts[0], workflowIds)));
        } else {
          var ruleFn = parseRule(ruleParts, workflowIds);
          rules.add(ruleFn);
        }
      }
      return new Workflow(id, rules);
    }

    public record Rule(char condition, int check, int value, int destination) {

      int apply(long[] part) {
        return switch (condition) {
          case '<' -> part[check] < value ? destination : NEXT;
          case '>' -> part[check] > value ? destination : NEXT;
          case '-' -> destination;
          default -> throw new IllegalStateException(STR."Unexpected condition: \{condition}");
        };
      }
    }

    private static Rule parseRule(String[] ruleParts, List<String> workflowIds) {
      var check = "xmas".indexOf(ruleParts[0].charAt(0));
      var condition = ruleParts[0].charAt(1);
      var value = Integer.parseInt(ruleParts[0].substring(2));
      var destination = workflowIdFromString(ruleParts[1], workflowIds);

      return new Rule(condition, check, value, destination);
    }

    private static int workflowIdFromString(String workflow, List<String> workflowIds) {
      return switch (workflow) {
        case "A" -> ACCEPTED;
        case "R" -> REJECTED;
        default -> workflowIds.indexOf(workflow);
      };
    }
  }
}
