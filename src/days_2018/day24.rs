// https://adventofcode.com/2018/day/24

use itertools::Itertools;

use crate::common::Solution;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DamageType {
    Radiation,
    Fire,
    Slashing,
    Bludgeoning,
    Cold,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Faction {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Group {
    index: usize,
    units: i64,
    damage: DamageType,
    weaknesses: Vec<DamageType>,
    immunities: Vec<DamageType>,
    hitpoints: i64,
    attack: i64,
    initiative: i64,
    target: Option<usize>,
    faction: Faction,
}

impl Group {
    fn effective_power(&self) -> i64 {
        self.units * self.attack
    }

    fn damage_modifier_vs(&self, target: &Self) -> i64 {
        if target.immunities.contains(&self.damage) {
            0
        } else if target.weaknesses.contains(&self.damage) {
            2
        } else {
            1
        }
    }

    fn parse(s: &str, faction: Faction) -> Vec<Group> {
        s.lines()
            .enumerate()
            .filter_map(|(i, line)| {
                line.parse::<Group>()
                    .map(|g| {
                        let mut group = g.clone();
                        group.faction = faction.clone();
                        group.index = i;
                        group
                    })
                    .ok()
            })
            .collect()
    }

    fn determine_target(&self, remaining_targets: &Vec<Group>) -> Option<(usize, Group)> {
        remaining_targets
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, e)| self.faction != e.faction)
            .filter(|(_, e)| self.damage_modifier_vs(e) > 0)
            .max_by(|(_, e1), (_, e2)| {
                let c = self
                    .damage_modifier_vs(e1)
                    .cmp(&self.damage_modifier_vs(e2));
                if c == std::cmp::Ordering::Equal {
                    let c = e1.effective_power().cmp(&e2.effective_power());
                    if c == std::cmp::Ordering::Equal {
                        e1.initiative.cmp(&e2.initiative)
                    } else {
                        c
                    }
                } else {
                    c
                }
            })
    }
}

fn attempt(immune_system: &Vec<Group>, infection: &Vec<Group>, boost: i64) -> i64 {
    let mut all: Vec<Group> = immune_system
        .iter()
        .map(|g| {
            let mut g = g.clone();
            g.attack += boost;
            g
        })
        .chain(infection.into_iter().cloned())
        .collect();

    while all.iter().any(|g| g.faction == Faction::ImmuneSystem)
        && all.iter().any(|g| g.faction == Faction::Infection)
    {
        all.sort_unstable_by(|a, b| {
            if a.effective_power() == b.effective_power() {
                b.initiative.cmp(&a.initiative)
            } else {
                b.effective_power().cmp(&a.effective_power())
            }
        });
        let mut possible_targets = all.clone();
        let original = all.clone();
        for g in all.iter_mut() {
            if let Some((i, target)) = g.determine_target(&mut possible_targets) {
                g.target = original.iter().position(|t| *t == target);
                possible_targets.remove(i);
            } else {
                g.target = None;
            }
        }

        let indices: Vec<usize> = all
            .iter()
            .map(|g| g.initiative)
            .enumerate()
            .sorted_by_key(|(_, initiative)| initiative.clone())
            .rev()
            .map(|(index, _)| index)
            .collect();

        let mut total_kills = 0;
        for index in indices.into_iter() {
            if let Some(target_index) = all[index].target {
                let damage = all[index].effective_power()
                    * all[index].damage_modifier_vs(&all[target_index]);
                let killed_units =
                    (damage / all[target_index].hitpoints).min(all[target_index].units);
                all[target_index].units -= killed_units;
                total_kills += killed_units;
            }
        }
        if total_kills == 0 {
            // Infinite battle. Immune system will not win.
            return 1;
        }

        all = all.into_iter().filter(|g| g.units > 0).collect();
    }
    let immune_system_won = all.iter().any(|g| g.faction == Faction::ImmuneSystem);

    let sign = if immune_system_won { -1 } else { 1 };
    sign * all.iter().map(|g| g.units).sum::<i64>()
}

pub fn solve(input: &str) -> Solution {
    // I originally wrote this in C and for this specific problem Rust is a lot clunkier.
    let (immune_system, infection) = input.split_once("\n\n").unwrap();
    let immune_system = Group::parse(immune_system, Faction::ImmuneSystem);
    let infection = Group::parse(infection, Faction::Infection);

    let p1 = attempt(&immune_system, &infection, 0);
    let mut boost = 1;
    let p2 = loop {
        let a = attempt(&immune_system, &infection, boost);
        if a < 0 {
            break -a;
        } else {
            boost = boost + 1;
        }
    };
    Solution::new(p1, p2)
}

impl FromStr for Group {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let units = parts[0].parse().map_err(|_| "Invalid unit count")?;
        let hitpoints = parts[4].parse().map_err(|_| "Invalid hitpoints")?;
        let initiative = parts[parts.len() - 1]
            .parse()
            .map_err(|_| "Invalid initiative")?;
        let damage = parts[parts.len() - 5]
            .parse()
            .map_err(|_| "Invalid damage type")?;
        let attack = parts[parts.len() - 6]
            .parse()
            .map_err(|_| "Invalid attack value")?;

        let mut is_weakness = false;
        let mut weaknesses = vec![];
        let mut immunities = vec![];
        for m in parts[7..(parts.len() - 11)].iter() {
            let m = m
                .replace("(", "")
                .replace(")", "")
                .replace(";", "")
                .replace(",", "");
            if m == "weak" {
                is_weakness = true;
            } else if m == "immune" {
                is_weakness = false;
            } else if let Some(t) = m.parse::<DamageType>().ok() {
                if is_weakness {
                    weaknesses.push(t);
                } else {
                    immunities.push(t);
                }
            }
        }
        Ok(Self {
            damage,
            weaknesses,
            immunities,
            hitpoints,
            attack,
            initiative,
            units,
            faction: Faction::ImmuneSystem,
            index: 0,
            target: None,
        })
    }
}

impl FromStr for DamageType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radiation" => Ok(Self::Radiation),
            "slashing" => Ok(Self::Slashing),
            "bludgeoning" => Ok(Self::Bludgeoning),
            "fire" => Ok(Self::Fire),
            "cold" => Ok(Self::Cold),
            _ => Err("unknown damage type"),
        }
    }
}
