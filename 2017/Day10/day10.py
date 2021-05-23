from collections import deque


lengths = deque(map(int, open("Day10/input").read().split(",")))

hash_len = 256
string = list(range(hash_len))

def get_strip(start, n):
    strip = []
    start %= hash_len
    
    while n != 0:
        strip.append(string[start])
        n -= 1
        start = (start + 1) % hash_len
        
    return reversed(strip)

def write_strip(start, strip):
    global string
    for i, num in enumerate(strip):
        string[(start + i) % hash_len] = num

pos = 0
skip_size =  0
while lengths:
    length = lengths.popleft()
    
    write_strip(pos, get_strip(pos, length))
    
    pos = (pos + length + skip_size) % hash_len
    skip_size += 1
    
    
print(string)
    
    
    
