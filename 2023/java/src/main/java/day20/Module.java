package day20;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public abstract class Module {

  public static final int HIGH = 1;
  public static final int LOW = 0;

  String id;
  final List<Module> destinations = new ArrayList<>();

  Router router;

  public void connect(Router router) {
    this.router = router;
  }

  public void reset() {
    // DO NOTHING;
  }

  public void addDestination(Module destination) {
    this.destinations.add(destination);
  }

  abstract void receive(Module from, int pulse);

  void send(Module from, Module to, int pulse) {
    this.router.send(from, to, pulse);
  }

  public String getId() {
    return id;
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append(STR."Module \{id}:");
    destinations.forEach(dest -> sb.append(STR."\n  - \{dest.id}"));
    return sb.toString();
  }

  public static class FlipModule extends Module {

    private boolean on;

    public FlipModule(String id) {
      this.id = id;
      on = false;
    }

    @Override
    public void receive(Module from, int pulse) {
      if (pulse == HIGH) {
        return;
      }

      int send = on ? LOW : HIGH;
      on = !on;
      destinations.forEach(d -> this.send(this, d, send));
    }
  }

  public static class ConjunctionModule extends Module {

    private final Map<Module, Integer> lastInputs;

    public ConjunctionModule(String id) {
      this.id = id;
      this.lastInputs = new HashMap<>();
    }

    public void addInput(Module input) {
      lastInputs.put(input, LOW);
    }

    @Override
    public void reset() {
      lastInputs.keySet().forEach(key -> lastInputs.put(key, LOW));
    }

    @Override
    public void receive(Module from, int pulse) {
      lastInputs.put(from, pulse);
      var send = lastInputs.values().stream().allMatch(p -> p == HIGH) ? LOW : HIGH;
      destinations.forEach(d -> this.send(this, d, send));
    }
  }

  public static class BroadcastModule extends Module {

    public BroadcastModule() {
      this.id = "broadcaster";
    }

    @Override
    public void receive(Module from, int pulse) {
      destinations.forEach(d -> this.send(this, d, pulse));
    }
  }

  public static class ButtonModule extends Module {

    public ButtonModule(Module broadcaster) {
      this.id = "button";
      this.addDestination(broadcaster);
    }

    public void push() {
      this.send(this, destinations.getFirst(), LOW);
    }

    @Override
    public void receive(Module from, int pulse) {
      throw new UnsupportedOperationException();
    }
  }

  public static class OutputModule extends Module {

    public OutputModule(String id) {
      this.id = id;
    }

    @Override
    void receive(Module from, int pulse) {
      // DO NOTHING
    }
  }
}
