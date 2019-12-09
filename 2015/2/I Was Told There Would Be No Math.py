def wrap(l, w, h):
    s = [2*l*w, 2*w*h, 2*h*l]
    return (sum(s) + min(s)/2, sum(sorted([l, w, h])[:2])*2 + (l*w*h))

parcels = [list(map(int, line.split('x'))) for line in open('input.txt')]

print(sum([wrap(*p)[0] for p in parcels]))
print(sum([wrap(*p)[1] for p in parcels]))

