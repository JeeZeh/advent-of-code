groups = open("input.txt").read().split("\n\n")

print(sum(len(set(group.replace("\n", ""))) for group in groups))
print(sum(all(ans in p for p in g.split()) for g in groups for ans in set(g)))

# Part 2 un-golfed
# total = 0
# for group in groups:
#     group_answers = set(group.replace("\n", ""))
#     total += sum(all(ans in person for person in group.split("\n")) for ans in group_answers)

# print(total)