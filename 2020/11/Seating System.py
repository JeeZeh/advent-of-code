seats = open("input.txt").read().splitlines()
seats = {(i, j):seats[j][i] for j in range(len(seats)) for i in range(len(seats[j]))}

dirs = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]

def get_surrounding(pos):
    surrounding = []
    
    for dir in dirs:
        check = (pos[0] - dir[0], pos[1] - dir[1])
        if tile := seats.get(check):
            surrounding.append(tile)
                
    return surrounding

def get_visible(pos):
    visible = []
    for dir in dirs:
        check = (pos[0] + dir[0], pos[1] + dir[1])
        while check in seats:
            if seats[check] != ".":
                visible.append(seats[check])
                break
            check = (check[0] + dir[0], check[1] + dir[1])
    return visible

def next_state(pos):
    surrounding = get_visible(pos)
    if seats[pos] == "L" and not "#" in surrounding:
        return "#"
    if seats[pos] == "#" and sum(s == "#" for s in surrounding) > 4:
        return "L"
    
    return seats[pos]

def print_grid(grid):
    print("\n".join(map("".join, grid)))
 

def step():
    new_seats = {}
    for pos in seats:
        new_seats[pos] = next_state(pos)
    return new_seats

states = [None]

# while states[-1] != seats:
#     states.append(seats)
#     seats = step()
    
# print(sum(x == "#" for x in seats.values()))

seats = open("input.txt").read().splitlines()
seats = {(i, j):seats[j][i] for j in range(len(seats)) for i in range(len(seats[j]))}

while states[-1] != seats:
    states.append(seats)
    seats = step()
    
print(sum(x == "#" for x in seats.values()))
