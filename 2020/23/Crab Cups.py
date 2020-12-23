cups = "193467258"

cups_list = list(map(int, cups))

cups = {}

mn, mx = 1, len(cups_list)

for i, c in enumerate(cups_list):
    cups[c] = cups_list[(i + 1) % len(cups_list)]


def move(current, cups):
    next_3 = {
        cups[current]: cups[cups[current]],
        cups[cups[current]]: cups[cups[cups[current]]],
        cups[cups[cups[current]]]: cups[cups[cups[cups[current]]]],
    }

    dest_cup = find_dest(current, cups, next_3)

    first_in_holding = cups[current]
    last_in_holding = cups[cups[cups[current]]]
    last_in_holding_points_to = cups[cups[cups[cups[current]]]]

    # Current now points to the what the last cup in holding points to
    cups[current] = last_in_holding_points_to

    # The last cup in holding now points to what the destination points to
    cups[last_in_holding] = cups[dest_cup]

    # The destination cup now points to the first cup in holding
    cups[dest_cup] = first_in_holding

    return cups


def find_dest(current, cups, holding):
    destination = current - 1
    while True:
        if destination < mn:
            destination = mx
        elif destination in holding:
            destination -= 1
        else:
            return destination



def part1(cups):
    global mn, mx
    mn, mx = 1, len(cups_list)

    p1_cups = cups.copy()
    current = cups_list[0]

    for _ in range(100):
        move(current, p1_cups)
        current = p1_cups[current]

    output = []
    next_ = p1_cups[1]
    while len(output) < len(p1_cups) - 1:
        output.append(str(next_))
        next_ = p1_cups[next_]

    print("".join(output))


def part2(cups):
    global mn, mx

    p2_cups = cups.copy()

    for x in range(mx + 1, 1_000_001):
        if x == 1_000_000:
            p2_cups[x] = cups_list[0]
        else:
            p2_cups[x] = x + 1

    p2_cups[cups_list[-1]] = mx + 1

    mn, mx = 1, 1_000_000

    current = cups_list[0]
    for _ in range(10_000_000):
        move(current, p2_cups)
        current = p2_cups[current]

    print(p2_cups[1], p2_cups[p2_cups[1]])
    print(p2_cups[1] * p2_cups[p2_cups[1]])


part1(cups)
part2(cups)