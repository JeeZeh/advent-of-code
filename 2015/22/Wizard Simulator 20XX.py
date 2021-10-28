from collections import deque
from copy import copy
from dataclasses import dataclass
from typing import List, Literal, Set, Tuple, Union
from textwrap import dedent


@dataclass
class Stats:
    hitpoints: int
    damage: int
    armour: int
    mana: int


@dataclass
class Effect:
    status: Union[
        Literal["armour"], Literal["damage"], Literal["mana"], Literal["heal"], None
    ]
    power: int
    duration: int

    def apply(
        self,
        attacker_hp: int,
        attacker_armour: int,
        attacker_mana: int,
        defender_hp: int,
    ) -> bool:
        if self.status == "armour":
            if self.duration == 1:
                attacker_armour = 0
            else:
                attacker_armour = self.power
        elif self.status == "damage":
            defender_hp -= self.power
        elif self.status == "mana":
            attacker_mana += self.power
        elif self.status == "heal":
            attacker_hp += self.power

        # Should we remove this effect?
        return (
            Effect(self.status, self.power, self.duration - 1),
            attacker_hp,
            attacker_armour,
            attacker_mana,
            defender_hp,
        )


@dataclass
class Spell:
    cost: int
    effects: list[Effect]
    instant: bool


spells: List[Tuple[str, Spell]] = [
    ("r", Spell(229, [Effect("mana", 101, 5)], False)),
    ("s", Spell(113, [Effect("armour", 7, 6)], False)),
    ("d", Spell(73, [Effect("damage", 2, 0), Effect("heal", 2, 0)], True)),
    ("p", Spell(173, [Effect("damage", 3, 6)], False)),
    ("m", Spell(53, [Effect("damage", 4, 0)], True)),
]


@dataclass
class Game:
    player: Stats
    boss: Stats
    tried_spells: Set[str]
    effects: List[Effect]
    player_attacks: bool
    total_mana_spent: int
    history: deque

    def __str__(self):
        return dedent(
            f"""
            Player: {self.player}
            Boss: {self.boss}
            Tried: {self.tried_spells}
            Effects: {self.effects}
            Mana Spent: {self.total_mana_spent}
            """
        )


best_game = 2000


def try_round(
    player_hp: int,
    player_armour: int,
    player_mana: int,
    boss_hp: int,
    boss_damage: int,
    effects: List[Effect],
    spell: Spell,
    total_mana_spent,
    path: str,
    hard=False,
):
    global best_game

    total_mana_spent += spell.cost
    player_mana -= spell.cost

    if spell.instant:
        for e in spell.effects:
            _, player_hp, player_armour, player_mana, boss_hp = e.apply(
                player_hp, player_armour, player_mana, boss_hp
            )

        # Game won because of instant effect
        if boss_hp <= 0:
            if total_mana_spent < best_game:
                best_game = total_mana_spent
            # This path is complete, backtrack
            # print("Player wins:", total_mana_spent, path)
            return
    else:
        effects += spell.effects

    ### BOSS TURN ###
    new_effects = []
    for e in effects:
        new_effect, player_hp, player_armour, player_mana, boss_hp = e.apply(
            player_hp, player_armour, player_mana, boss_hp
        )
        if new_effect.duration > 0:
            new_effects.append(new_effect)

    effects = new_effects

    # Game won because of instant effect
    if boss_hp <= 0:
        if total_mana_spent < best_game:
            best_game = total_mana_spent
        # This path is complete, backtrack
        # print("Player wins:", total_mana_spent, path)
        return

    player_hp -= max(1, boss_damage - player_armour)

    # Game lost because of boss attack
    if player_hp <= 0:
        return

    ### PLAYER TURN ###
    if hard:
        player_hp -= 1
        # Game lost because of HP bleed (hard mode)
        if player_hp <= 0:
            return

    new_effects = []
    for e in effects:
        new_effect, player_hp, player_armour, player_mana, boss_hp = e.apply(
            player_hp, player_armour, player_mana, boss_hp
        )
        if new_effect.duration > 0:
            new_effects.append(new_effect)

    effects = new_effects

    # Game ended because of passive effect (negative effects only apply to boss)
    if boss_hp <= 0:
        if total_mana_spent < best_game:
            best_game = total_mana_spent
        # print("Player wins:", total_mana_spent, path)
        # This path is complete, backtrack
        return

    # Choose spell
    for name, spell in spells:
        if spell.cost > player_mana:
            continue

        # Don't explore this path, it's already more expensive!
        if spell.cost + total_mana_spent >= best_game:
            continue

        # Are any of the spell's effects already active?
        if not spell.instant:
            statuses = set(e.status for e in effects)
            if any(e.status in statuses for e in spell.effects):
                continue

        try_round(
            player_hp,
            player_armour,
            player_mana,
            boss_hp,
            boss_damage,
            copy(effects),
            spell,
            total_mana_spent,
            path + name,
            hard,
        )


def find_best_game(player: Stats, boss: Stats, hard=False):
    for name, spell in spells:
        try_round(
            player.hitpoints,
            player.armour,
            player.mana,
            boss.hitpoints,
            boss.damage,
            [],
            spell,
            0,
            name,
            hard,
        )

    print(f"Best game ({'hard'if hard else 'normal'}):", best_game)


# find_best_game(Stats(10, 0, 0, 250), Stats(14, 8, 0, 0))
best_game = 9999
find_best_game(Stats(50, 0, 0, 500), Stats(71, 10, 0, 0))

best_game = 9999
find_best_game(Stats(50, 0, 0, 500), Stats(71, 10, 0, 0), True)