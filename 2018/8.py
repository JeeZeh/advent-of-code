def main():
    with open('test.txt') as f:
        line = f.readlines()

    print(line)

    for i in range(len(line)):


def process_node(i, line):
    num_children = int(line[i])
    i += 1
    num_meta = int(line[i])
    if num_children > 0:
        process_node()



main()