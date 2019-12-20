import re

def valid(string):
    m = re.match
print(len([s for s in open('input.txt') if valid(s)]))