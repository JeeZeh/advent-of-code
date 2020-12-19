def parse(equation):
    parts = []
    buffer = ""

    for char in equation:
        if char == " ":
            if buffer:
                parts.append(buffer)
                buffer = ""
        elif char in ("(", ")"):
            if buffer:
                parts.append(buffer)
                buffer = ""
            parts.append(char)
        else:
            buffer += char

    if buffer:
        parts.append(buffer)

    return parts


def is_int(x):
    try:
        int(x)
        return True
    except:
        return False


equations = map(parse, open("real.txt").read().splitlines())

# This is a nasty function that is a bad mix of recursion, iteration, and state
# passing. Probably a ton of redundancy too, but it works
# Basic idea is to split off into nested solutions at certain operators.
# Part 1 meant splitting at ( and returning at )
# Part 2 meant the above AND splitting at * and returning at either end or )
#   The behaviour changes between reaching ) in a regular nest vs. a *-based nest
#   so a mult_nest was used to indicate it. This is because when in a * nest, on 
#   return you still have a value you need to perform a * with, whereas in a ) nest
#   you're done with all operations for that nest and can move forward without ops
def solve(equation, index=0, mult_nest=False):
    result = 0
    operand = "+"

    while index < len(equation):
        part = equation[index]
        if part == "+":
            operand = part
        elif part == "*":
            operand = part
            part, index = solve(equation, index + 1, mult_nest=True)
        elif part == "(":
            part, index = solve(equation, index + 1)
            pass
        elif part == ")":
            if mult_nest:
                return result, index - 1
            else:
                break

        if is_int(part):
            if operand == "+":
                result += int(part)
            else:
                result *= int(part)

        index += 1

    return result, index


# print(solve(parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")))

sum_ = 0
for equation in equations:
    print(solve(equation))
    sum_ += solve(equation)[0]

print(sum_)
