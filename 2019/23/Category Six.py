from intcode import Intcode
from collections import defaultdict, deque
network = []
NAT = None
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
    NAT = None
    while True:
        idle = True
        for addr, nic in enumerate(network):
            sending = None
            if queue[addr]:
                data = queue[addr].popleft()
                t1 = nic.send(data[0])
                t2 = nic.send(data[1])
                if t2:
                    sending = t2 # t1 doesn't block, so t2 could receive back a sending pack
            else:
                sending = nic.send(-1)

            if sending:
                X = next(nic)
                Y = next(nic)
                if sending == 255:
                    NAT = (X, Y)
                else:
                    queue[sending].append((X, Y))
        if NAT:
            X, Y = NAT 
            print(f"NAT Sending: {X}, {Y}")
            queue[0].append((X, Y))


tape = list(map(int, open("input.txt").readline().split(",")))
init_NICs(tape.copy())
cycle()
            
