// https://adventofcode.com/2015/day/21

use crate::common::Solution;
use itertools::Itertools;

const STORE: &str = "Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

#[derive(Debug)]
struct ItemForSale {
    cost: u16,
    damage: u16,
    armor: u16,
}

fn parse_shop(shop: &str) -> Vec<ItemForSale> {
    shop.lines()
        .skip(1)
        .map(|line| {
            let (armor, damage, cost) = line
                .split_ascii_whitespace()
                .rev()
                .take(3)
                .collect_tuple()
                .unwrap();
            let cost = cost.parse::<u16>().unwrap();
            let damage = damage.parse::<u16>().unwrap();
            let armor = armor.parse::<u16>().unwrap();
            ItemForSale {
                cost,
                damage,
                armor,
            }
        })
        .collect()
}

pub fn solve(input: &str) -> Solution {
    let (weapons, armors, rings) = STORE
        .split("\n\n")
        .map(|shop| parse_shop(shop))
        .collect_tuple()
        .unwrap();

    let (boss_hp, boss_damage, boss_armor) = input
        .lines()
        .map(|line| {
            line.split(": ")
                .skip(1)
                .next()
                .unwrap()
                .parse::<u16>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let mut p1 = u16::MAX;
    let mut p2 = 0;
    for weapon_index in 0..weapons.len() {
        let weapon = &weapons[weapon_index];
        for armor_index in 0..(armors.len() + 1) {
            let armor = armors.get(armor_index);
            for ring_1_index in 0..(rings.len() + 1) {
                let ring_1 = rings.get(ring_1_index);
                for ring_2_index in 0..(rings.len() + 1) {
                    if ring_2_index == ring_1_index {
                        continue;
                    }
                    let ring_2 = rings.get(ring_2_index);

                    let attack = weapon.damage
                        + ring_1.map(|r| r.damage).unwrap_or(0)
                        + ring_2.map(|r| r.damage).unwrap_or(0);

                    let defense = armor.map(|a| a.armor).unwrap_or(0)
                        + ring_1.map(|r| r.armor).unwrap_or(0)
                        + ring_2.map(|r| r.armor).unwrap_or(0);

                    let cost = weapon.cost
                        + armor.map(|a| a.cost).unwrap_or(0)
                        + ring_1.map(|r| r.cost).unwrap_or(0)
                        + ring_2.map(|r| r.cost).unwrap_or(0);

                    let player_turns = if attack <= boss_armor {
                        boss_hp
                    } else {
                        boss_hp.div_ceil(attack - boss_armor)
                    };
                    let boss_turns = if boss_damage <= defense {
                        100
                    } else {
                        100u16.div_ceil(boss_damage - defense)
                    };
                    let win = player_turns <= boss_turns;

                    if win && cost < p1 {
                        p1 = cost
                    };
                    if !win && cost > p2 {
                        p2 = cost
                    };
                }
            }
        }
    }

    Solution::new(p1, p2)
}
