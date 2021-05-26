from asyncio.windows_utils import pipe


def parse_pipes(p):
    p = p.split(" <-> ")
    id_1 = int(p[0])
    joins = map(int, p[1].split(", "))

    return id_1, joins


programs = map(parse_pipes, open("Day12/input").read().splitlines())

pipes = {p: c for p, c in programs}


def explore(id_, visited):
    if id_ in visited:
        return visited

    visited.add(id_)

    for c in pipes[id_]:
        visited |= explore(c, set(visited))

    return visited


components = {}
group = 0
for p in pipes:
    if p in components:
        continue

    group += 1
    components |= {c: group for c in explore(p, set())}

print(max(components.values()))
