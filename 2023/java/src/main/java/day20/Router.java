package day20;

import static day20.Module.HIGH;
import static day20.Module.LOW;

import java.util.ArrayDeque;
import java.util.Deque;

public class Router {

  long highPulses = 0;
  long lowPulses = 0;

  boolean sentRx = false;

  public void reset() {
    lowPulses = 0;
    highPulses = 0;
  }

  record Packet(Module from, Module to, int pulse) {

  }

  final Deque<Packet> packets = new ArrayDeque<>();

  void send(Module from, Module to, int pulse) {
//    System.out.println(STR."Queuing \{pulse} from \{from.id} to \{to.id}");
//    System.out.println(STR."\{from.id} -\{pulse == HIGH ? "high" : "low"}-> \{to.id}");

    packets.add(new Packet(from, to, pulse));
  }

  void process() {
    while (!packets.isEmpty()) {
      var packet = packets.poll();
//      System.out.println(
//          STR."\{packet.from.id} -\{packet.pulse == HIGH ? "high" : "low"}-> \{packet.to.id}");
      if (packet.pulse == LOW) {
        lowPulses++;
      } else {
        highPulses++;
      }

      if (packet.pulse == LOW && packet.to.id.equals("rx")) {
        sentRx = true;
      }
      packet.to.receive(packet.from, packet.pulse);
    }
  }
}
