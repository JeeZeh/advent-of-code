import re
from collections import deque


def bfs(tunnels, start, targets):
    dist = {start: 0}
    seen = {start}
    q = deque([start])
    while q and any(t not in dist for t in targets):
        p = q.popleft()
        for x in tunnels[p]:
            if x not in seen:
                seen.add(x)
                dist[x] = dist[p] + 1
                q.append(x)
    return dist


def find_paths(dist, rates, t):
    pressures = []
    paths = []
    stack = [(t, 0, ['AA'])]
    while stack:
        t, p, path = stack.pop()
        cur = path[-1]
        new = []
        for n, d in dist[cur].items():
            if d > t - 2 or n in path:
                continue
            tt = t - d - 1
            pp = p + rates[n] * tt
            s = tt, pp, path + [n]
            new.append(s)
        if new:
            stack.extend(new)
        else:
            pressures.append(p)
            # paths always start at AA, so no need to keep first location
            paths.append(path[1:])
    return pressures, paths


def solve(data: str):
    rates = {}
    tunnels = {}
    for line in data.splitlines():
        _, valve, *_ = line.split()
        r = int(line.split("; ")[0].split("rate=")[1])
        if r:
            rates[valve] = r
        m = re.search(r'valves? (.+)$', line).group(1)
        tunnels[valve] = m.split(', ')
    # Part One
    dist = {}
    for start in ('AA', *rates):
        dist[start] = {}
        d = bfs(tunnels, start, rates)
        for r in rates:
            if r != start and r in d:
                dist[start][r] = d[r]

    p, _ = find_paths(dist, rates, 30)
    print(dist)
    
    # Part Two
    x = list(zip(*find_paths(dist, rates, 26)))
    p, paths = zip(*sorted(x, reverse=True))
    print(p[0], len(p))
    i, j = 0, 1
    while any(x in paths[j] for x in paths[i]):
        j += 1
    ans = p[i] + p[j]  # lower bound
    j_max = j  # since p[i] can only decrease, j cannot exceed this
    print(paths[:3], p[:3])
    for i in range(1, j_max):
        for j in range(i + 1, j_max + 1):
            if any(x in paths[j] for x in paths[i]):
                continue
            ans = max(ans, p[i] + p[j])
    print(ans)


if __name__ == '__main__':
    solve(open("inputs\\day16.txt").read())
