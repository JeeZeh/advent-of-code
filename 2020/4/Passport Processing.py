from string import hexdigits, digits

required_keys = {
    "byr": lambda x: 1920 <= int(x) <= 2002,
    "iyr": lambda x: 2010 <= int(x) <= 2020,
    "eyr": lambda x: 2020 <= int(x) <= 2030,
    "hgt": lambda x: "cm" in x and 150 <= int(x[:-2]) <= 193 or "in" in x and 59 <= int(x[:-2]) <= 76,
    "hcl": lambda x: x[0] == "#" and all(c in hexdigits for c in x[1:]),
    "ecl": lambda x: x in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
    "pid": lambda x: len(x) == 9 and all(c in digits for c in x),
}

get_creds = lambda x: {pair.split(":")[0]:pair.split(":")[1] for pair in x.replace("\n", " ").split()}
kv_passports = [get_creds(raw) for raw in open("input.txt").read().split("\n\n")]

part_1, part_2 = 0, 0
for password in kv_passports:
    if set(required_keys.keys()).issubset(password.keys()):
        part_1 += 1
        part_2 += all([required_keys[k](v) for k, v in password.items() if k != "cid"])

print(part_1, part_2)
