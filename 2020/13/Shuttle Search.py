import math


earliest, busses_line = open("input.txt")
earliest = int(earliest)
busses = list(map(int, filter(lambda x: x != "x", busses_line.split(","))))

nearest = min((bus - (earliest % bus), bus) for bus in busses)
print("Part 1:", nearest[0] * nearest[1])


busses = [(i, int(b)) for i, b in enumerate(busses_line.split(",")) if b != "x"]

# This took way too long to figure out, I had been trying fancier things like lcm/gcd...
# I had this idea of offsetting all the busses to line up with the first one and using LCM to get the first point that happens.
# This is sort of how the solution works, but the offset is used in a different way than I had first tried,
# and LCM isn't used (or maybe a principle of it actually is, not sure).
#
# Solution:
#   a, b are 2 periods (busses, waves, etc.)
#   Period 'a' has an offset i.e. the cycle begins at this time (-10 minutes, +3000 minutes, etc)
#   Keep incrementing the period, 1 cycle at a time, beginning at the offset
#   If we find that at any cycle, period b starts a distace of 'diff' from period a
#   we return when this happens, the period at which it happens, and when this wave should begin (offset)
#
#   We can then add more periods by 'stacking' previous periods and calling the function again.
#   This works quickly by 'skipping' all the smaller periods between each significant pattern we are trying to find
#   e.g. if we know "B appears 1 minute after A" every 10 minutes, and we want when C appears after this pattern,
#        we don't need to check every occurrence of B, only the occurences of B where it appears 1 minute after A (the pattern).
#
#   Example
#     1. Get the period where B appears 1 minute after A (we now know when this will happen)
#     2. Get the period where C appears 4 minutes after the A where (B appears 1 minute after A)
#     3. Get the period where D appears 7 minutes after the A where (C appears 4 minutes after the A where (B appears 1 minute after A))
#     4 -> len(busses) - 1: ...
#     len(busses): The final evaluation gives us the time when the busses appear in the pattern we want


def get_wave_params(a, b, diff, offset):
    check = a + offset
    while True:
        if (check + diff) % b == 0:
            return check, a * b, check - a * b

        check = check + a


freq, offset, cycle_start = busses[0][1], 0, 0
for bus in busses[1:]:
    cycle_start, freq, offset = get_wave_params(freq, bus[1], bus[0], offset)

print("Part 2:", cycle_start)
