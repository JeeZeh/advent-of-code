from collections import defaultdict

def parse_foods():
    allergens = defaultdict(set)
    ingredients = set()
    foods = []
    
    with open("real.txt") as f:
        for l in f:
            ings, alls = l.strip().split(" (contains ")
            ingredients |= set(ings.split())
            foods.append(ings.split())
            for allergen in alls[:-1].split(", "):
                if allergen in allergens:
                    allergens[allergen] = allergens[allergen].intersection(set(ings.split()))
                else:
                    allergens[allergen] = set(ings.split())

    return foods, ingredients, allergens

def get_next_unique_ingredient(allergens):
    for allergen, ingredients in allergens.items():
        if len(ingredients) == 1:
            return allergen, list(ingredients)[0]
        
def reduce_allergen_matches(to_remove, allergens: dict):
    for allergen in allergens.keys():
        if to_remove in allergens[allergen]:
            allergens[allergen].remove(to_remove)
    
    return {a:i for a, i in allergens.items() if i}

def count_leftovers(foods, leftovers):
    count = 0
    for food_ingredients in foods:
        count += sum(l in food_ingredients for l in leftovers)
    
    return count
    
def part_1():
    foods, ingredients, allergens = parse_foods()

    assigned_allergens = {}
    to_filter = allergens.copy()
    
    while match := get_next_unique_ingredient(to_filter):
        al, ing = match
        assigned_allergens[al] = ing
        allergens = reduce_allergen_matches(ing, to_filter)

    leftover_ingredients = ingredients.difference(assigned_allergens.values())
    print(count_leftovers(foods, leftover_ingredients))
    
    return assigned_allergens

def part_2(assigned):
    dangerous = ",".join(assigned[k] for k in sorted(assigned.keys()))
    print(dangerous)

part_2(part_1())