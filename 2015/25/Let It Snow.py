from typing import Tuple


def generate_codes(seed: int) -> Tuple[int, int, int]:
    row, col = 1, 1

    current = seed
    while True:
        yield row, col, current

        if row == 1:
            row = col + 1
            col = 1
        else:
            col += 1
            row -= 1

        current = (current * 252533) % 33554393


seen = set()
for row, col, code in generate_codes(20151125):
    if code in seen:
        print(len(seen), row, col, code)
        break
    
    seen.add(code)
    if (row, col) == (2947, 3029):
        print("Part 1:", code)
        break