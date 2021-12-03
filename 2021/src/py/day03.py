reports = open("input").read().splitlines()


def rot_90(l):
    return list(zip(*l[::-1]))


def most_common_bit(col):
    return "1" if col.count("1") >= col.count("0") else "0"


def generate_life_metrics(reports, oxy):
    for col_num in range(len(reports[0])):
        new_reports = []
        common = most_common_bit(rot_90(reports)[col_num])
        for bin_ in reports:
            if oxy and bin_[col_num] == common:
                new_reports.append(bin_)
            elif not oxy and bin_[col_num] != common:
                new_reports.append(bin_)

        reports = new_reports
        if len(reports) == 1:
            break

    return new_reports[0]


gamma_rate = "".join(most_common_bit(col) for col in rot_90(reports))
epsilon_rate = "".join("1" if c == "0" else "0" for c in gamma_rate)

print("Part 1:", int(gamma_rate, 2) * int(epsilon_rate, 2))

oxygen_reports = generate_life_metrics(reports, True)
co2_reports = generate_life_metrics(reports, False)

print("Part 2:", int(oxygen_reports, 2) * int(co2_reports, 2))
