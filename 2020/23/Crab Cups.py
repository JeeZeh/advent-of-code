cups = "389125467"


class clist(list):
    def __getitem__(self, k):
        if isinstance(k, slice):
            start = k.start if k.start else 0
            stop = k.stop if k.stop else len(self)
            return [self[n % len(self)] for n in range(start, stop)]
        return list.__getitem__(self, k % len(self))


cups = clist(map(int, cups))

mx = max(cups)
mn = min(cups)

def take(s, n, cups):
    holding = cups[s : s + n]
    cups = cups[s+n:len(cups)+s]

    return holding, cups


def insert(to_insert, s, cups):
    return clist(cups[:s] + to_insert + cups[s:])


def find_dest(current, cups, holding):
    destination = current - 1
    while True:
        if destination in cups:
            return cups.index(destination), destination
        if destination < mn:
            destination = mx
        if destination in holding:
            destination -= 1


cup = cups[-1]
for _ in range(100):
    current_index = (cups.index(cup) + 1) % len(cups)
    cup = cups[current_index]
    holding, cups = take(current_index + 1, 3, cups)
    insert_index, number = find_dest(cup, cups, holding)
    cups = insert(holding, insert_index + 1, cups)

print("".join(map(str,cups[cups.index(1) + 1 : cups.index(1) + len(cups)])))
