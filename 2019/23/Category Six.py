from intcode import Intcode
from collections import defaultdict, deque
network = []
queue = defaultdict(deque)

def init_NICs(tape):
    for addr in range(50):
        NIC = Intcode()
        NIC = NIC.init(tape)
        NIC.send(addr)
        network.append(NIC)

def queue_packet(addr, data):
    queue[addr].append(data)
    
def cycle():
    while True:
        idle = True
        for addr, nic in enumerate(network):
            sending = None
            if queue[addr]:
                idle = False
                data = queue[addr].popleft()
                t1 = nic.send(data[0])
                t2 = nic.send(data[1])
                if t2:
                    sending = t2
            else:
                if not sending:
                    sending = nic.send(-1)
            if sending:
                X = next(nic)
                Y = next(nic)
                if sending == 255:
                    queue[255].append((X, Y))
                else:
                    queue_packet(sending, (X, Y))
        if idle:
            X, Y = queue[255].pop()
            print(f"NAT Sending: {X}, {Y}")
            queue[0].append((X, Y))


tape = list(map(int, open("input.txt").readline().split(",")))
init_NICs(tape)
cycle()
            
