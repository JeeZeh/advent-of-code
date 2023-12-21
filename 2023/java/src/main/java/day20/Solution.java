package day20;

import day20.Module.BroadcastModule;
import day20.Module.ButtonModule;
import day20.Module.ConjunctionModule;
import day20.Module.FlipModule;
import day20.Module.OutputModule;
import java.io.IOException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import lib.Input;
import lib.Number;

public class Solution {

  public static void main(String[] args) throws IOException {
    List<Module> modules = createModules(Input.lines("day20/input.txt").toList());
    var broad = modules.stream().filter(m -> m.getId().equals("broadcaster")).findFirst().get();
    ButtonModule button = new ButtonModule(broad);

    Router router = new Router();
    modules.forEach(m -> m.connect(router));
    button.connect(router);

    pressButton(router, button, 1000);
    System.out.println(STR."Part 1: \{router.getLowPulses() * router.getHighPulses()}");
    System.out.println(STR."Part 2: \{findRxInputPulse(router, modules, button)}");
  }

  public static long findRxInputPulse(Router router, List<Module> modules, ButtonModule button) {
    var rxInputs = getInputsForId(modules, "rx");
    var proxyRxInputs = getInputsForId(modules, rxInputs.getFirst().id);
    return proxyRxInputs.stream().map(input -> getCycleForHighBit(router, modules, button, input))
        .reduce(Number::lcm).get();
  }

  public static long getCycleForHighBit(Router router, List<Module> modules, ButtonModule button,
      Module module) {
    router.reset();
    modules.forEach(Module::reset);

    int firstInstance = 0;
    while (router.getHighPulses(module) == 0) {
      button.push();
      router.process();
      firstInstance += 1;
    }

    int secondInstance = firstInstance;
    while (router.getHighPulses(module) == 1) {
      button.push();
      router.process();
      secondInstance += 1;
    }

    return secondInstance - firstInstance;
  }

  public static void pressButton(Router router, ButtonModule button, int times) {
    for (int i = 0; i < times; i++) {
      button.push();
      router.process();
    }
  }

  public static List<Module> getInputsForId(List<Module> modules, String id) {
    return modules.stream().filter(module -> module.id.equals(id)).findFirst().get().inputs;
  }

  public static List<Module> createModules(List<String> lines) {
    Map<String, Module> modules = new HashMap<>();
    for (String line : lines) {
      var parts = line.split(" -> ");
      Module module;
      String rawId = parts[0];
      if (parts[0].equals("broadcaster")) {
        module = new BroadcastModule();
      } else {
        var id = parts[0].substring(1);
        if (parts[0].startsWith("%")) {
          module = new FlipModule(id);
        } else {
          module = new ConjunctionModule(id);
        }
      }
      modules.put(rawId, module);
    }

    for (String line : lines) {
      var parts = line.split(" -> ");
      var sourceModule = modules.get(parts[0]);
      Arrays.stream(parts[1].split(", ")).forEach(dest -> {
        var maybeDestModule = modules.values().stream()
            .filter(module -> module.getId().equals(dest)).findAny();

        Module destModule;
        if (maybeDestModule.isPresent()) {
          destModule = maybeDestModule.get();
        } else {
          destModule = new OutputModule(dest);
          modules.put(dest, destModule);
        }
        sourceModule.addDestination(destModule);
        destModule.addInput(sourceModule);

      });
    }

    return modules.values().stream().toList();
  }
}
