from typing import Generator

def generate_a(seed, factor):
    while True:
        seed = (seed * factor) % 2147483647
        if seed % 4 == 0:
            yield seed & 0xFFFF
        

def generate_b(seed, factor):
    while True:
        seed = (seed * factor) % 2147483647
        if seed % 8 == 0:
            yield seed & 0xFFFF
        
a = generate_a(591, 16807)
b = generate_b(393, 48271)

print(sum(next(a) == next(b) for _ in range(5_000_000)))