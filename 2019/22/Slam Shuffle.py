from collections import deque

deck = [c for c in range(119315717514047)]

def deal():
    global deck
    deck = deck[::-1]
    return deck

def cut(n):
    global deck
    deck = deck[n:] + deck[:n] 
    return deck


def incr_deal(n):
    global deck
    deck = deck[::-1]
    table = [0 for _ in range(len(deck))]
    i = 0
    while deck:
        table[i%len(table)] = deck.pop()
        i+=n

    deck = table
    return deck

for line in open("input.txt"):
    if "increment" in line:
        incr_deal(int(line.split(" ")[-1]))
    elif "cut" in line:
        cut(int(line.split(" ")[-1]))
    else:
        deal()

print(deck.index(2019))