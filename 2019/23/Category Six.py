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
    delivered = None
    while True:
        for addr, nic in enumerate(network):
            if computers[addr].is_waiting():
                if queue[addr]:
                    data = queue[addr].popleft()
                    if addr == 0:
                        if delivered == data[1]:
                            print("YIKES", delivered)
                            return  
                        else:
                            delivered = data[1]
                    nic.send(data[0])
                    nic.send(data[1])
                else:
                    nic.send(-1)
                
            if computers[addr].is_sending():
                dest = next(nic)
                next(nic)
                X = next(nic)
                next(nic)
                Y = next(nic)
                next(nic)
                if dest == 255:
                    NAT = (X, Y)
                else:
                    queue[dest].append((X, Y))

        if is_idle():
            X, Y = NAT 
            queue[0].append((X, Y))


tape = list(map(int, open("input.txt").readline().split(",")))
init_NICs(tape)
cycle()
            
