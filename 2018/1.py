
def find():
    with open('1.txt') as f:
        lines = f.readlines()

    lines = [line.strip() for line in lines]

    freq = 0
    all_freqs = {0: 1}

    while True:
        for line in lines:
            if(line[0] == '-'):
                freq -= int(line[1:])
            else:
                freq += int(line[1:])
            
            if(freq in all_freqs):
                return freq
                
            else:
                all_freqs[freq] = 1


print(find())
