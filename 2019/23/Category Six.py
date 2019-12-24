from intcode import Intcode
from collections import defaultdict, deque
network = []
computers = []
NAT = None
queue = defaultdict(deque)

def init_NICs(tape):
    for addr in range(50):
        computer = Intcode()
        NIC = computer.init(tape.copy())
        NIC.send(addr)
        network.append(NIC)
        computers.append(computer)

def queue_packet(addr, data):
    queue[addr].append(data)

def is_idle():
    for i, c in enumerate(computers):
        if not c.is_waiting() or queue[i]:
            return False
    print("All waiting")
    return True 
    
def cycle():
    NAT = None
    while True:
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
        if is_idle():
            X, Y = NAT 
            print(f"NAT Sending: {X}, {Y}")
            queue[0].append((X, Y))


tape = list(map(int, open("input.txt").readline().split(",")))
init_NICs(tape)
cycle()
            
