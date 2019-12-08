modules = list(map(int, open('input.txt')))

total_fuel = 0
for mass in modules:
    fuel = (mass//3) - 2
    # Sum fuel
    total_fuel += fuel
print(f"Part 1: {total_fuel}")

total_fuel = 0
for mass in modules:
    fuel_mass = (mass//3) - 2
    extra_fuel = (fuel_mass//3) - 2
    while extra_fuel > 0:
        fuel_mass += extra_fuel
        extra_fuel = (extra_fuel//3) - 2
    total_fuel += fuel_mass

print(f"Part 2: {total_fuel}")
