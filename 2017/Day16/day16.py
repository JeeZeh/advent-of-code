from string import ascii_lowercase

class InverseDict(dict):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.inverse = {v: k for k, v in self.items()}
        
    def __setitem__(self, key, val):
        super().__setitem__(key, val)
        self.inverse[val] = key
    
dance_size = 16
letters = InverseDict({l: i for i, l in enumerate(ascii_lowercase[:dance_size])})
    
def spin(c):
    global letters
    
    for l, pos in letters.items():
        letters[l] = (pos + c) % dance_size


def swap(a: str, b: str):
    global letters

    pos_a, pos_b = letters[a], letters[b]
    letters[a] = pos_b
    letters[b] = pos_a
    
def exchange(a: int, b: int):
    global letters
    
    swap(letters.inverse[a], letters.inverse[b])

def get_operation(s: str):
    op = s[0]
    
    if op == "s":
        return spin, (int(s[1:]),)
    
    if op == "x":
        return exchange, tuple(map(int, s[1:].split("/")))
    
    if op == "p":
        return swap, (s[1], s[3])

dance_routine = list(map(get_operation, open("input").read().split(",")))

def get_period(routine) -> int:
    global letters
    
    letters = InverseDict({l: i for i, l in enumerate(ascii_lowercase[:dance_size])})
    
    states = set()
    current_state = "".join(map(lambda x: x[0], sorted(letters.items(), key=lambda x: x[1])))
    while current_state not in states:
        states.add(current_state)
        
        for operation, args in dance_routine:
            operation(*args)
            
        current_state = "".join(map(lambda x: x[0], sorted(letters.items(), key=lambda x: x[1])))

    return len(states)
        

def dance(times):
    dance_routine = list(map(get_operation, open("input").read().split(",")))
    
    period = get_period(dance_routine)

    for _ in range(times % period):
        for operation, args in dance_routine:
            operation(*args)

        
    return "".join(map(lambda x: x[0], sorted(letters.items(), key=lambda x: x[1])))

print(dance(1_000_000_000))