from string import ascii_lowercase
from typing import List

blocked = {ascii_lowercase.index("i"), ascii_lowercase.index("o"), ascii_lowercase.index("l")}

# Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
# Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
# Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.


def increasing_triplets(password: List[int]) -> bool:
    for i in range(len(password) - 2):
        if password[i] + 1 == password[i + 1] and password[i + 1] + 1 == password[i + 2]:
            return True

    return False


def non_overlapping_pairs(password: List[int]):
    pairs = 0
    i = 0
    while i < len(password) - 1:
        if password[i] == password[i + 1]:
            pairs += 1
            i += 2
        else:
            i += 1
            
        if pairs == 2:
            return True

def valid_passwords(seed: str):
    password = [ascii_lowercase.index(s) for s in seed]

    while True:
        # print(''.join(map(ascii_lowercase.__getitem__, password)))
    
        found_blocked = False
        for i, n in enumerate(password):
            if found_blocked:
                password[i] = 0
            elif n in blocked:
                password[i] += 1
                found_blocked = True
            elif i == len(password) - 1:
                password[i] += 1
            
        for i in range(len(password)-1, -1, -1):
            if password[i] == len(ascii_lowercase):
                password[i] = 0
                password[i-1] += 1
                
        if non_overlapping_pairs(password) and increasing_triplets(password):
            try:
                yield ''.join(map(ascii_lowercase.__getitem__, password))
            except IndexError:
                print(password)


gen = valid_passwords("hepxcrrq")

print("Part 1:", next(gen))
print("Part 2:", next(gen))
