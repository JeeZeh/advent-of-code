package day20;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public abstract class Module {

  public static final int HIGH = 1;
  public static final int LOW = 0;

  String id;
  List<Module> destinations;

  abstract void receive(Module from, int pulse);


  public static class FlipModule extends Module {

    private boolean on;

    public FlipModule(String id, List<Module> destinations) {
      on = false;
      this.destinations = destinations;
    }

    @Override
    public void receive(Module from, int pulse) {
      if (pulse == HIGH) {
        return;
      }

      int send = on ? LOW : HIGH;
      on = !on;
      destinations.forEach(d -> d.receive(send));
    }
  }

  public static class ConjunctionModule extends Module {

    private final Map<Module, Integer> lastInputs;

    public ConjunctionModule(String id, List<Module> destinations) {
      this.lastInputs = new HashMap<>();
      this.destinations = destinations;
    }

    @Override
    public void receive(Module from, int pulse) {
      if (pulse == HIGH) {
        return;
      }

      int send = on ? LOW : HIGH;
      on = !on;
      destinations.forEach(d -> d.receive(send));
    }
  }
}
