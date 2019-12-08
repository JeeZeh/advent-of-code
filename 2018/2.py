from collections import defaultdict

def checksum():
    with open('2.txt') as f:
        lines = f.readlines()

    lines = [line.strip() for line in lines]

    twos, threes = 0, 0

    for line in lines:
        unique = {}
        unique = defaultdict(lambda:0,unique)
        for letter in line:
            unique[letter] += 1
        if 2 in unique.values():
            twos+=1
        if 3 in unique.values():
            threes+=1
        
    print(twos * threes)
    

def protoype():
    with open('2.txt') as f:
        lines = f.readlines()

    lines = [line.strip() for line in lines]

    for i in range(len(lines)):
        for j in range(i+1, len(lines)):
            diff = ""
            for k in range(len(lines[0])):
                if lines[i][k] != lines[j][k]:
                    diff+= lines[i][k]
                
            if len(diff) == 1:
                print("".join(lines[i].split(diff)))
                return

# checksum()
protoype()