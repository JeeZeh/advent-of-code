import math 
from pprint import pprint
from collections import defaultdict
d = {}
storage = defaultdict(int)
ore = []
MAX = 1_000_000_000_000

for line in open('input.txt'):
    line = line.rstrip()
    rs = line.split(" => ")[0].split(", ")
    pr = line.split(" => ")[1].split(" ")
    d[pr[1]] = ([(int(r.rstrip().split(" ")[0]), r.strip().split(" ")[1]) for r in rs], int(pr[0]))


def check_storage(mineral, need):
    take_from_store = min(storage[mineral], need)
    need -= take_from_store
    storage[mineral] -= take_from_store
    return need

def make(mineral, qty):
    ingredients = d[mineral][0]
    produces = d[mineral][1]
    need = check_storage(mineral, qty)
    batches = math.ceil(need/produces)
    storage[mineral] += (batches*produces) - need
    for i in ingredients:
        if i[1] == "ORE":
            ore.append((batches*i[0], mineral, batches*produces))
        else:
            make(i[1], batches*i[0])

def get_ore(fuel_amount):
    global storage, ore
    storage = defaultdict(int)
    ore = []
    make("FUEL", fuel_amount)
    return sum([o[0] for o in ore])

def find_max(a, b):
    if b >= a:
        mid = int(a + (b-a)/2)
        if get_ore(mid) < MAX:
            if get_ore(mid+1) > MAX:
                return mid
            else:
                return find_max(mid+1, b)
        elif get_ore(mid) > MAX:
            if get_ore(mid-1) < MAX:
                return mid - 1
            else:
                return find_max(a, mid-1)
    else:
        return -1

#### PART 1 ####

print(get_ore(1))

#### PART 2 ####

print(find_max(0, MAX))