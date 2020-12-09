data = list(map(int, open("input.txt")))

window = 25


def get_subsum(s, slice_):
    numset = set(slice_)
    for n in slice_:
        if s - n in numset:
            return n, s - n


def find_subset_sum(s, data):
    for slice_length in range(2, len(data)):
        for i in range(0, len(data)):
            slice_ = data[i : i + slice_length]
            if s == sum(slice_):
                return slice_


for i in range(window, len(data)):
    if not get_subsum(data[i], data[i - window : i]):
        print(data[i])
        if sol := find_subset_sum(data[i], data[:i]):
            print(min(sol) + max(sol))