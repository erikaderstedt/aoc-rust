// https://adventofcode.com/2018/day/15

use crate::{
    common::Solution,
    grid::{Grid, GridElement, Position},
};
use itertools::Itertools;
use pathfinding::num_traits::Euclid;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Arena {
    Wall,
    Floor,
    Goblin(u8),
    Elf(u8),
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

fn num_elves(arena: &Grid<Arena>) -> usize {
    arena
        .locations
        .iter()
        .filter(|a| match a {
            Arena::Elf(_) => true,
            _ => false,
        })
        .count()
}

fn flood_search<F>(start_index: usize, arena: &Grid<Arena>, found: F) -> Option<usize>
where
    F: Fn(usize) -> bool,
{
    let cols = arena.cols;
    if found(start_index + 1)
        || found(start_index - 1)
        || found(start_index - cols)
        || found(start_index + cols)
    {
        return Some(start_index);
    }

    // let (row, col) = start_index.div_rem_euclid(&cols);
    let mut travel = vec![0usize; arena.rows * cols];
    let mut d = 1;
    // Fill with edge of most recent. We could keep track of all filled positions in the previous round instead of
    // iterating over the grid.
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
                if found(index + 1)
                    || found(index - 1)
                    || found(index - cols)
                    || found(index + cols)
                {
                    // let (row2, col2) = index.div_rem_euclid(&cols);
                    // println!(
                    //     "idx {} ({},{}) -> found idx {} ({},{})",
                    //     start_index, row, col, index, row2, col2
                    // );

                    return Some(index);
                }
            }
        }
        d = d + 1;
    }
    // println!("idx {} ({},{}) -> did not find any", start_index, row, col);
    None
}

fn combat<const NO_ELF_CAN_DIE: bool>(
    arena: &mut Grid<Arena>,
    elf_attack_power: &u8,
) -> (usize, usize) {
    let mut num_full_rounds_completed = 0;
    let mut anyone_moved_or_died_last_round = true;
    loop {
        // Get actors (will automatically be in reading order, since the grid is iterated that way)
        let mut actor_positions: Vec<Position> = arena
            .positions()
            .filter(|p| match arena.get(p).unwrap() {
                Arena::Elf(_) | Arena::Goblin(_) => true,
                _ => false,
            })
            .collect();
        actor_positions.reverse(); // For popping

        let mut anyone_moved_or_died_this_round = false;
        while let Some(mut actor) = actor_positions.pop() {
            let is_elf = match arena.get(&actor).unwrap() {
                Arena::Elf(_) => true,
                Arena::Goblin(_) => false,
                Arena::Wall => panic!("Actor position should not be a wall!"),
                Arena::Floor => continue, // killed before it could act.
            };

            let opponent_positions: Vec<Position> = arena
                .positions()
                .filter(|p| match arena.get(p).unwrap() {
                    Arena::Goblin(_) => is_elf,
                    Arena::Elf(_) => !is_elf,
                    _ => false,
                })
                .collect();

            if opponent_positions.len() == 0 {
                return (
                    num_full_rounds_completed,
                    arena.locations.iter().map(|a| a.hp() as usize).sum(),
                );
            }

            // Is any in range?
            if anyone_moved_or_died_last_round
                && !opponent_positions
                    .iter()
                    .any(|p| p.manhattan_distance(&actor) == 1)
            {
                // If no, move
                // Search for an index that is adjacent to an enemy.
                let actor_index = actor.row * arena.cols + actor.column;
                if let Some(destination_index) =
                    flood_search(actor_index, arena, |index| match arena.locations[index] {
                        Arena::Elf(_) if !is_elf => true,
                        Arena::Goblin(_) if is_elf => true,
                        _ => false,
                    })
                {
                    // Search from that spot to an index adjacent to the actor.
                    let index_to_move_to =
                        flood_search(destination_index, arena, |index| index == actor_index)
                            .unwrap();
                    // Perform move
                    arena.locations[index_to_move_to] = arena.locations[actor_index].clone();
                    arena.locations[actor_index] = Arena::Floor;
                    let (row, column) = index_to_move_to.div_rem_euclid(&arena.cols);
                    actor = Position { row, column };
                    anyone_moved_or_died_this_round = true;
                }
            }
            if let Some(weakest_adjacent_enemy) = opponent_positions
                .iter()
                .filter(|p| p.manhattan_distance(&actor) == 1)
                .sorted_by_key(|p| arena.get(p).unwrap().hp())
                .next()
            {
                // If yes, then attack
                let result = arena
                    .get(weakest_adjacent_enemy)
                    .unwrap()
                    .attacked(if is_elf { &elf_attack_power } else { &3 });

                arena.set(weakest_adjacent_enemy, result.clone());
                if result == Arena::Floor {
                    if !is_elf && NO_ELF_CAN_DIE {
                        return (0, 0);
                    }
                    // Remove this from actor_positions if it was still there.
                    if let Some(index) = actor_positions
                        .iter()
                        .position(|a| a == weakest_adjacent_enemy)
                    {
                        actor_positions.remove(index);
                    }

                    anyone_moved_or_died_this_round = true;
                }
            }
        }
        num_full_rounds_completed = num_full_rounds_completed + 1;
        // If noone moved and noone died, skip movement check next iteration.
        anyone_moved_or_died_last_round = anyone_moved_or_died_this_round;
    }
}

// Reasonable start value is 4 x 3 = 12, since it is likely that an elf gets surrounded by four goblins.
// For part 2 we can also quit as soon as an elf dies.
const GUESS_AT_NEEDED_ATTACK_POWER: u8 = 12;

pub fn solve(input: &str) -> Solution {
    let arena: Grid<Arena> = Grid::load(input);

    let mut attack_power = 3u8;
    let num_elves_initally = num_elves(&arena);

    let (p1_combat_rounds, remaining_hp) = combat::<false>(&mut arena.clone(), &attack_power);
    let p1 = p1_combat_rounds * remaining_hp;
    attack_power = GUESS_AT_NEEDED_ATTACK_POWER;
    let p2 = loop {
        let mut a = arena.clone();
        let (result, remaining_hp) = combat::<true>(&mut a, &attack_power);

        if remaining_hp > 0 && num_elves_initally == num_elves(&a) {
            break result * remaining_hp;
        }
        attack_power = attack_power + 1;
    };
    Solution::new(p1, p2)
}
