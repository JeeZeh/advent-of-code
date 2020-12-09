data = list(map(int, open("input.txt")))

window = 25


def get_subsum(s, slice_):
    numset = set(slice_)
    for n in slice_:
        if s - n in numset:
            return n, s - n

def generate_subset_sums():
    subsets = {}
    
    for slice_length in range(2, len(data)):
        for i in range(0, len(data)):
            slice_ = data[i: i+slice_length]
            subsets[sum(slice_)] = slice_
            
    return subsets

subset_sums = generate_subset_sums()

for i in range(window, len(data)):
    if not get_subsum(data[i], data[i-window: i]):
        print(data[i])
        if sol := subset_sums.get(data[i]):
            print(min(sol) + max(sol))