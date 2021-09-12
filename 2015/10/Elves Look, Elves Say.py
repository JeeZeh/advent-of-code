def say(phrase: str):
    output = ""
    
    current, count = phrase[0], 1
    
    for n in phrase[1:]:
        if n == current:
            count += 1
        else:
            output += f"{count}{current}"
            current = n
            count = 1
            
    output += f"{count}{current}"
    return output
    
phrase = "3113322113"
for _ in range(40):
    phrase = say(phrase)
    
print("Part 1:", len(phrase))

for _ in range(10):
    phrase = say(phrase)
    
print("Part 2:", len(phrase))