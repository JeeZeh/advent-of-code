from sympy import simplify
from sympy.core.expr import Expr
from sympy.core.function import expand
from sympy.ntheory import modular
from sympy.polys.polytools import cancel, factor
from sympy.simplify.powsimp import powsimp

deck = [c for c in range(10007)]


def deal(deck):
    deck = deck[::-1]
    return deck


def reverse_deal(x: int, decksize: int):
    return f"({decksize} - {x} - 1)"


def cut(deck, n):
    deck = deck[n:] + deck[:n]
    return deck


def reverse_cut(n: int, x: int, decksize: int):
    return f"({x} + {n})"


def incr_deal(deck, n):
    deck = deck[::-1]
    table = [0 for _ in range(len(deck))]
    i = 0
    while deck:
        table[i % len(table)] = deck.pop()
        i += n

    deck = table
    return deck


def reverse_incr_deal(n: int, x: int, decksize: int):
    return f"({x} * pow({n}, -1, {decksize}))"


inputs = list(open("input.txt"))


def position_of_card(card, deck, shuffles=5):
    for _ in range(shuffles):
        for line in inputs:
            if "increment" in line:
                deck = incr_deal(deck, int(line.split(" ")[-1]))
            elif "cut" in line:
                deck = cut(deck, int(line.split(" ")[-1]))
            else:
                deck = deal(deck)

    return deck.index(card)


def generate_shuffle_string(decksize):
    # Follow each step in reverse to undo the shuffling

    equation = "p"
    for line in reversed(inputs):
        if "increment" in line:
            equation = reverse_incr_deal(int(line.split(" ")[-1]), equation, decksize)
        elif "cut" in line:
            equation = reverse_cut(int(line.split(" ")[-1]), equation, decksize)
        else:
            equation = reverse_deal(equation, decksize)

    # Remove the pow() function calls
    for i in range(1, 100):
        pattern = f"pow({i}, -1, {decksize})"

        equation = equation.replace(pattern, str(eval(pattern)))

    return equation
    # return f"({equation}) % {decksize}"


def mod_atoms(expression: Expr, mod):
    expression_string = str(expression)

    for atom in expression.atoms():
        if atom.is_number:
            expression_string = expression_string.replace(str(atom), str(atom % mod))

    return expression_string


# https://math.stackexchange.com/a/453108
def modular_pow(base, exponent, modulus):
    result = 1
    while exponent > 0:
        if exponent % 2 == 1:
            result = (result * base) % modulus
        exponent = exponent >> 1
        base = (base * base) % modulus
    return result


def get_card_at_p_after_n_shuffles(p, n, decksize):
    single_shuffle = generate_shuffle_string(decksize)
    simplified_single_shuffle = simplify(single_shuffle)

    coeff = simplified_single_shuffle.coeff("p") % decksize
    constant = [term for term in simplified_single_shuffle.args if not term.free_symbols][0] % decksize

    # "ax+b" N times is the same as "(a^2)x + ab + b" N/2 times
    shuffle_n = lambda a, x, b: ((a * x) + b) % decksize

    times = n
    while times > 0:
        if times % 2 == 1:
            times -= 1
            p = shuffle_n(coeff, p, constant)
        else:
            times //= 2
            constant = (coeff * constant + constant) % decksize
            coeff = (coeff ** 2) % decksize
    return p


print("Part 1:", position_of_card(2019, list(range(10007)), 1))
print("Part 2:",get_card_at_p_after_n_shuffles(2020, 101741582076661, 119315717514047))
