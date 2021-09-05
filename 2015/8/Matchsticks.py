from json import dumps

memory = open("input.txt").read().splitlines()
decoded = list(map(eval, memory))
encoded = list(map(dumps, memory))

print(sum(map(len, memory)) - sum(map(len, decoded)))
print(sum(map(len, encoded)) - sum(map(len, memory)))
