// https://adventofcode.com/2018/day/15

use crate::{
    common::Solution,
    grid::{Grid, GridElement},
};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Arena {
    Wall,
    Floor,
    Goblin(u8),
    Elf(u8),
}

fn flood_search<F>(start_index: usize, arena: &Grid<Arena>, found: F) -> Option<usize>
where
    F: Fn(usize) -> bool,
{
    let cols = arena.cols;
    if found(start_index) {
        return Some(start_index);
    }

    let mut travel = vec![0u8; arena.rows * cols];
    let mut d = 1;
    // Fill with edge of most recent.
    travel[start_index] = d;
    let mut work_needed = true;
    while work_needed {
        // All positions which are 0 and where the arena floor is zero, and that are adjacent to the previous round.
        work_needed = false;
        for index in 0..travel.len() {
            if travel[index] == 0
                && arena.locations[index] == Arena::Floor
                && (travel[index - 1] == d  // This indexing is safe because there are no floor spaces on the edge.
                    || travel[index + 1] == d
                    || travel[index - cols] == d
                    || travel[index + cols] == d)
            {
                travel[index] = d + 1;
                work_needed = true;
                if found(index) {
                    return Some(index);
                }
            }
        }
        d = d + 1;
    }
    None
}

fn combat(arena: &mut Grid<Arena>, elf_attack_power: &u8, stop_if_elf_dies: bool) -> Option<usize> {
    let mut round = 0;
    let cols = arena.cols;
    loop {
        // Get actors (will automatically be in reading order, since the grid is iterated that way)
        let mut actor_indices: Vec<usize> = arena.indices_matching(|p| match p {
            Arena::Elf(_) | Arena::Goblin(_) => true,
            _ => false,
        });
        actor_indices.reverse(); // For popping later

        while let Some(mut actor_index) = actor_indices.pop() {
            let is_elf = match arena.locations[actor_index] {
                Arena::Elf(_) => true,
                Arena::Goblin(_) => false,
                Arena::Wall => panic!("Actor position should not be a wall!"),
                Arena::Floor => continue, // killed before it could act.
            };

            let opponent_indices: Vec<usize> = arena.indices_matching(|p| p.is_enemy(is_elf));

            if opponent_indices.len() == 0 {
                let total_hp = arena
                    .locations
                    .iter()
                    .map(|a| a.hp() as usize)
                    .sum::<usize>();
                return Some(round * total_hp);
            }

            // Is any in range?
            if !opponent_indices
                .iter()
                .any(|&p| arena.indices_are_adjacent(actor_index, p))
            {
                // If no, move
                // Search for an index that is adjacent to an enemy.
                if let Some(dest) = flood_search(actor_index, arena, |index| {
                    arena.locations[index - 1].is_enemy(is_elf)
                        || arena.locations[index + 1].is_enemy(is_elf)
                        || arena.locations[index - cols].is_enemy(is_elf)
                        || arena.locations[index + cols].is_enemy(is_elf)
                }) {
                    // Search from that spot to an index adjacent to the actor. This is to ensure we step
                    // to the first step on the correct path.
                    if let Some(index_to_move_to) = flood_search(dest, arena, |index| {
                        arena.indices_are_adjacent(index, actor_index)
                    }) {
                        // Perform move
                        arena.locations[index_to_move_to] = arena.locations[actor_index].clone();
                        arena.locations[actor_index] = Arena::Floor;
                        actor_index = index_to_move_to;
                    }
                }
            }
            // Is any in range now after moving?
            if let Some(weakest_adjacent_enemy) = opponent_indices
                .into_iter()
                .filter(|&p| arena.indices_are_adjacent(actor_index, p))
                .sorted_by_key(|&p| arena.locations[p].hp())
                .next()
            {
                // If yes, then attack
                let ap = if is_elf { elf_attack_power } else { &3 };
                let result = arena.locations[weakest_adjacent_enemy].attacked(ap);

                arena.locations[weakest_adjacent_enemy] = result.clone();
                if result == Arena::Floor {
                    if !is_elf && stop_if_elf_dies {
                        // !is_elf = "attacker is not an elf, meaning attacker is a goblin, meaning the victim is an elf"
                        return None;
                    }
                    // Remove this from actor_positions if it was still there.
                    if let Some(index) = actor_indices
                        .iter()
                        .position(|&a| a == weakest_adjacent_enemy)
                    {
                        actor_indices.remove(index);
                    }
                }
            }
        }
        round = round + 1;
    }
}

// Reasonable start value is 4 x 3 = 12, since it is likely that an elf gets surrounded by four goblins.
// For part 2 we can also quit as soon as an elf dies.
const GUESS_AT_NEEDED_ATTACK_POWER: u8 = 12;

pub fn solve(input: &str) -> Solution {
    let arena: Grid<Arena> = Grid::load(input);

    let p1 = combat(&mut arena.clone(), &3, false).unwrap();
    let mut attack_power = GUESS_AT_NEEDED_ATTACK_POWER;
    let p2 = loop {
        let mut a = arena.clone();
        if let Some(outcome) = combat(&mut a, &attack_power, true) {
            break outcome;
        }
        attack_power = attack_power + 1;
    };
    Solution::new(p1, p2)
}

impl Arena {
    fn hp(&self) -> u8 {
        match self {
            Arena::Elf(hp) => hp.clone(),
            Arena::Goblin(hp) => hp.clone(),
            _ => 0,
        }
    }
    fn attacked(&self, attack_power: &u8) -> Self {
        match self {
            Arena::Elf(hp) if hp > attack_power => Arena::Elf(hp - attack_power),
            Arena::Goblin(hp) if hp > attack_power => Arena::Goblin(hp - attack_power),
            _ => Arena::Floor,
        }
    }

    fn is_enemy(&self, is_elf: bool) -> bool {
        match self {
            Arena::Elf(_) if !is_elf => true,
            Arena::Goblin(_) if is_elf => true,
            _ => false,
        }
    }
}

impl GridElement for Arena {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Arena::Wall),
            '.' => Some(Arena::Floor),
            'G' => Some(Arena::Goblin(200)),
            'E' => Some(Arena::Elf(200)),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Arena::Elf(_) => 'E',
            Arena::Floor => '.',
            Arena::Goblin(_) => 'G',
            Arena::Wall => '█',
        }
    }
}
