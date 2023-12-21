package day20;

import static day20.Module.LOW;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

public class Router {

  Map<Module, AtomicInteger> lowPulses = new HashMap<>();
  Map<Module, AtomicInteger> highPulses = new HashMap<>();

  public void reset() {
    lowPulses.clear();
    highPulses.clear();
  }

  public int getLowPulses() {
    return lowPulses.values().stream().mapToInt(AtomicInteger::intValue).sum();
  }

  public int getHighPulses() {
    return highPulses.values().stream().mapToInt(AtomicInteger::intValue).sum();
  }

  public int getLowPulses(Module module) {
    return lowPulses.getOrDefault(module, new AtomicInteger(0)).get();
  }

  public int getHighPulses(Module module) {
    return highPulses.getOrDefault(module, new AtomicInteger(0)).get();
  }

  record Packet(Module from, Module to, int pulse) {

  }

  final Deque<Packet> packets = new ArrayDeque<>();

  void send(Module from, Module to, int pulse) {
    packets.add(new Packet(from, to, pulse));
  }

  void process() {
    while (!packets.isEmpty()) {
      var packet = packets.poll();
      if (packet.pulse == LOW) {
        lowPulses.computeIfAbsent(packet.from, k -> new AtomicInteger()).incrementAndGet();
      } else {
        highPulses.computeIfAbsent(packet.from, k -> new AtomicInteger()).incrementAndGet();
      }

      packet.to.receive(packet.from, packet.pulse);
    }
  }
}
