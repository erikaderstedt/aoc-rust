// https://adventofcode.com/2015/day/22

use crate::common::Solution;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spell {
    Shield,
    Poison,
    Recharge,
    MagicMissile,
    Drain,
}

impl Spell {
    fn cost(&self) -> i64 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    hard_mode: bool,

    hp: i64,
    mana: i64,
    shield_active: i64,
    recharge_active: i64,
    poison_active: i64,

    boss_hp: i64,
    attack: i64,
}

impl State {
    fn new(boss_hp: i64, attack: i64, hard_mode: bool) -> Self {
        Self {
            hard_mode,
            hp: 50,
            mana: 500,
            shield_active: 0,
            poison_active: 0,
            recharge_active: 0,
            attack,
            boss_hp,
        }
    }

    fn handle_effects(&mut self) {
        if self.shield_active > 0 {
            self.shield_active -= 1;
        }
        if self.recharge_active > 0 {
            self.mana += 101;
            self.recharge_active -= 1;
        }
        if self.poison_active > 0 {
            self.boss_hp -= 3;
            self.poison_active -= 1;
        }
    }

    // Returns mana spent by player.
    // None if the spell could not be cast (not enough mana or already active)
    // or if we go below 1 hit point.
    fn simulate_player_and_boss_turns(&mut self, spell: &Spell) -> Option<i64> {
        if self.hard_mode {
            self.hp -= 1;
        }

        if self.mana < spell.cost() {
            return None;
        }

        // Player turn
        self.handle_effects();

        // Cast spell
        self.mana -= spell.cost();
        if self.mana < 0 {
            return None;
        };
        match spell {
            Spell::Recharge => {
                if self.recharge_active > 0 {
                    return None;
                } else {
                    self.recharge_active = 5;
                }
            }
            Spell::Shield => {
                if self.shield_active > 0 {
                    return None;
                } else {
                    self.shield_active = 6;
                }
            }
            Spell::Poison => {
                if self.poison_active > 0 {
                    return None;
                } else {
                    self.poison_active = 6;
                }
            }
            Spell::MagicMissile => self.boss_hp -= 4,
            Spell::Drain => {
                self.boss_hp -= 2;
                self.hp += 2;
            }
        }

        // Boss turn
        self.handle_effects();

        let armor = if self.shield_active > 0 { 7 } else { 0 };
        let damage = if armor >= self.attack {
            1
        } else {
            self.attack - armor
        };
        self.hp -= damage;
        if self.hp <= 0 && self.boss_hp > 0 {
            return None;
        }

        Some(spell.cost())
    }
}

fn combat(initial_state: &State) -> i64 {
    let possible_actions = [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Poison,
        Spell::Recharge,
        Spell::Shield,
    ];

    dijkstra(
        initial_state,
        |state| {
            let new_states: Vec<(State, i64)> = possible_actions
                .iter()
                .filter_map(|spell| {
                    let mut new_state = state.clone();
                    if let Some(mana_spent) = new_state.simulate_player_and_boss_turns(spell) {
                        Some((new_state, mana_spent))
                    } else {
                        None
                    }
                })
                .collect();
            // println!("{:?} -> {:?}", state, new_states);
            new_states
        },
        |p| p.boss_hp <= 0,
    )
    .unwrap()
    .1
}

pub fn solve(input: &str) -> Solution {
    let (boss_hp, attack) = input
        .lines()
        .map(|line| line.split(' ').last().unwrap().parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    let p1 = combat(&State::new(boss_hp, attack, false));
    let p2 = combat(&State::new(boss_hp, attack, true));

    Solution::new(p1, p2)
}
