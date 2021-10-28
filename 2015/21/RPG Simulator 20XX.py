import copy
from dataclasses import dataclass
from itertools import combinations
from typing import Iterator, List, Tuple


@dataclass
class Item:
    name: str
    cost: int
    damage: int
    armour: int


@dataclass
class Stats:
    hitpoints: int
    damage: int
    armour: int


@dataclass
class Build:
    stats: Stats
    cost: int


weapons = [
    Item("Dagger", 8, 4, 0),
    Item("Shortsword", 10, 5, 0),
    Item("Warhammer", 25, 6, 0),
    Item("Longsword", 40, 7, 0),
    Item("Greataxe", 74, 8, 0),
]

armours = [
    Item("Leather", 13, 0, 1),
    Item("Chainmail", 31, 0, 2),
    Item("Splintmail", 53, 0, 3),
    Item("Bandedmail", 75, 0, 4),
    Item("Platemail", 102, 0, 5),
]

rings = [
    Item("Damage +1", 25, 1, 0),
    Item("Damage +2", 50, 2, 0),
    Item("Damage +3", 100, 3, 0),
    Item("Defense +1", 25, 0, 1),
    Item("Defense +2", 40, 0, 2),
    Item("Defense +3", 80, 0, 3),
]

NONE = Item("None", 0, 0, 0)


def weapon_choices():
    return weapons


def armour_choices():
    return armours + [NONE]


def ring_choices() -> List[Tuple[Item, Item]]:
    return list(combinations(rings, 2)) + [(NONE, i) for i in rings] + [(NONE, NONE)]


def generate_builds() -> Iterator[Build]:
    for weapon in weapon_choices():
        for armour in armour_choices():
            for left_ring, right_ring in ring_choices():
                yield Build(
                    Stats(
                        100,
                        weapon.damage + left_ring.damage + right_ring.damage,
                        armour.armour + left_ring.armour + right_ring.armour,
                    ),
                    weapon.cost + armour.cost + left_ring.cost + right_ring.cost,
                )


def player_wins(build: Build) -> bool:
    boss = Stats(104, 8, 1)  # Puzzle input
    player = copy.copy(build.stats)

    player_attacks = True

    while boss.hitpoints > 0 and player.hitpoints > 0:
        attacker, defender = (player, boss) if player_attacks else (boss, player)
        damage = max(1, attacker.damage - defender.armour)
        defender.hitpoints -= damage

        # Take turns
        player_attacks = not player_attacks

    return boss.hitpoints < player.hitpoints


builds = list(generate_builds())
cheapest_winning_build = min(filter(player_wins, builds), key=lambda b: b.cost)
print("Part 1:", cheapest_winning_build)

most_expensive_losing_build = max(filter(lambda b: not player_wins(b), builds), key=lambda b: b.cost)
print("Part 2:", most_expensive_losing_build)
