pkey_card, pkey_door = map(int, open("real.txt"))

subject = 7
mod = 20201227

def crack(subject, target):
    loops = 0
    value = 1
    while value != target:
        value = (value * subject) % mod
        loops += 1
    return loops

def get_encryption(pkey, loop_size):
    value = 1
    for _ in range(loop_size):
        value = (pkey * value) % mod

    return value


loop_size = crack(subject, pkey_card)
private_key = get_encryption(pkey_door, loop_size)

print(private_key)