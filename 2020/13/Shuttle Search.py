import math


earliest, busses_line = open("input.txt")
earliest = int(earliest)
busses = list(map(int, filter(lambda x: x != "x", busses_line.split(","))))
              
nearest = min((bus - (earliest % bus), bus) for bus in busses)
print(nearest[0] * nearest[1])


diff = 1
busses = []
for bus in busses_line.split(","):
    if bus == "x":
        diff += 1
    else:
        busses.append((bus, diff))
        diff = 1
        
print(busses)

def get_period_with_difference(a, b, d):
    period = 0
    cycle = 1
    while (b*cycle) % a != d:
        cycle += 1
    return int(((cycle*b)-d) / a)
    
print(get_period_with_difference(7,13,1))
print(get_period_with_difference(11, 59, 3))
print(get_period_with_difference(48, 31, 2))
print(get_period_with_difference(9, 19, 1))
print(math.lcm(2, 9, 48, 11))