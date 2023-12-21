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

public class Solution {

  public static void main(String[] args) throws IOException {
    List<String> lines = Input.lines("day20/input.txt").toList();
    List<Module> modules = createModules(lines);
    Router router = new Router();
    modules.forEach(m -> m.connect(router));

    var broadcaster = modules.stream().filter(m -> m.getId().equals("broadcaster")).findFirst()
        .get();
    ButtonModule button = new ButtonModule(broadcaster);
    button.connect(router);

    pressButton(router, button, 1000);
    System.out.println(STR."Part 1: \{router.lowPulses * router.highPulses}");

    modules.forEach(Module::reset);
    router.reset();

    // TODO: Part 2
  }

  public static void pressButton(Router router, ButtonModule button, int times) {
    for (int i = 0; i < times; i++) {
      button.push();
      router.process();
    }
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
        if (ConjunctionModule.class.isAssignableFrom(destModule.getClass())) {
          ((ConjunctionModule) destModule).addInput(sourceModule);
        }
      });
    }

    return modules.values().stream().toList();
  }
}
