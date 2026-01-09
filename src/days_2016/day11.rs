// https://adventofcode.com/2016/day/11

use crate::common::Solution;
use pathfinding::prelude::astar;

const CHOOSE_2: [u8; 10] = [
    0b00011, 0b00101, 0b01001, 0b10001, 0b00110, 0b01010, 0b10010, 0b01100, 0b10100, 0b11000,
];
const CHOOSE_1: [u8; 5] = [0b10000, 0b01000, 0b00100, 0b00010, 0b00001];
const FLOORS: usize = 4;
const ELEMENTS: usize = 5;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    generators: [u8; FLOORS],
    microchips: [u8; FLOORS],
    elevator: usize, // State fits in u64
}

impl State {
    fn ready(&self) -> bool {
        self.generators[FLOORS - 1].count_ones() as usize == ELEMENTS
            && self.microchips[FLOORS - 1].count_ones() as usize == ELEMENTS
    }

    // This is the minimum bound. And also the actual answer for some people's
    // inputs.
    fn minimum_distance(&self) -> usize {
        let nums: Vec<usize> = self
            .generators
            .iter()
            .zip(self.microchips.iter())
            .map(|(g, m)| (g.count_ones() + m.count_ones()) as usize)
            .collect();
        (0..FLOORS)
            .map(|floor| 2 * nums.iter().take(floor + 1).sum::<usize>() - 3)
            .sum::<usize>()
    }

    fn is_fried(generators: u8, microchips: u8) -> bool {
        let chips_without_generator = (microchips ^ generators) & microchips;
        generators > 0 && chips_without_generator > 0
    }

    fn add_states<const N: usize>(
        &self,
        choices: &[u8; N],
        states: &mut Vec<State>,
        to_floor: usize,
    ) {
        let m = self.microchips[self.elevator];
        let g = self.generators[self.elevator];

        states.extend(
            choices
                .iter()
                .filter(|&choice| {
                    (g & choice).count_ones() == choice.count_ones()
                        && !Self::is_fried(g ^ choice, m)
                        && !Self::is_fried(
                            self.generators[to_floor] | choice,
                            self.microchips[to_floor],
                        )
                })
                .map(|&choice| {
                    let mut generators = self.generators.clone();
                    generators[self.elevator] ^= choice;
                    generators[to_floor] |= choice;
                    Self {
                        generators,
                        microchips: self.microchips.clone(),
                        elevator: to_floor,
                    }
                }),
        );
        states.extend(
            choices
                .iter()
                .filter(|&choice| {
                    (m & choice).count_ones() == choice.count_ones()
                        && !Self::is_fried(g, m ^ choice)
                        && !Self::is_fried(
                            self.generators[to_floor],
                            self.microchips[to_floor] | choice,
                        )
                })
                .map(|&choice| {
                    let mut microchips = self.microchips.clone();
                    microchips[self.elevator] ^= choice;
                    microchips[to_floor] |= choice;
                    Self {
                        generators: self.generators.clone(),
                        microchips: microchips,
                        elevator: to_floor,
                    }
                }),
        );
    }

    fn successors(&self) -> Vec<(State, usize)> {
        // Never bring a full pair down.
        let m = self.microchips[self.elevator];
        let g = self.generators[self.elevator];

        let mut states = vec![];

        let nm = m.count_ones();
        let ng = g.count_ones();
        assert!(nm + ng > 0);

        if self.elevator < FLOORS - 1 {
            // A. Move two generators or two microchips.
            self.add_states(&CHOOSE_2, &mut states, self.elevator + 1);
            self.add_states(&CHOOSE_1, &mut states, self.elevator + 1);

            // B. Move a pair. Only consider the first filled pair.
            let floor_above_has_unpaired_chips = (self.generators[self.elevator + 1]
                ^ self.microchips[self.elevator + 1])
                & self.microchips[self.elevator + 1]
                > 0;
            if !floor_above_has_unpaired_chips && m & g > 0 {
                let choice = 1u8 << ((m & g).trailing_zeros() as u8);
                let mut microchips = self.microchips.clone();
                let mut generators = self.generators.clone();
                generators[self.elevator] ^= choice;
                microchips[self.elevator] ^= choice;
                generators[self.elevator + 1] |= choice;
                microchips[self.elevator + 1] |= choice;
                states.push(Self {
                    generators,
                    microchips,
                    elevator: self.elevator + 1,
                });
            }
        }

        // If the lower floor is empty we don't need to consider moving there.
        let lower_floor_empty = self.elevator == 0
            || (self.microchips[self.elevator - 1] == 0 && self.generators[self.elevator - 1] == 0);

        if !lower_floor_empty {
            self.add_states(&CHOOSE_2, &mut states, self.elevator - 1);
            self.add_states(&CHOOSE_1, &mut states, self.elevator - 1);

            // B. Move a pair. Only consider the first filled pair.
            let floor_above_has_unpaired_chips = (self.generators[self.elevator - 1]
                ^ self.microchips[self.elevator - 1])
                & self.microchips[self.elevator - 1]
                > 0;
            if !floor_above_has_unpaired_chips && m & g > 0 {
                let choice = 1u8 << ((m & g).trailing_zeros() as u8);
                let mut microchips = self.microchips.clone();
                let mut generators = self.generators.clone();
                generators[self.elevator] ^= choice;
                microchips[self.elevator] ^= choice;
                generators[self.elevator - 1] |= choice;
                microchips[self.elevator - 1] |= choice;
                states.push(Self {
                    generators,
                    microchips,
                    elevator: self.elevator - 1,
                });
            }
        }

        states.into_iter().map(|s| (s, 1)).collect()
    }
}

fn bring_up_equipment(input: &str) -> usize {
    let types_in_input = ["promethium", "cobalt", "curium", "ruthenium", "plutonium"];

    let mut generators = [0; FLOORS];
    let mut microchips = [0; FLOORS];
    for (floor, line) in input.lines().enumerate() {
        for type_index in 0..types_in_input.len() {
            if line.contains(&format!("{} generator", types_in_input[type_index])) {
                generators[floor] |= 1 << type_index;
            }
            if line.contains(&format!(
                "{}-compatible microchip",
                types_in_input[type_index]
            )) {
                microchips[floor] |= 1 << type_index;
            }
        }
    }

    let initial_state = State {
        generators,
        microchips,
        elevator: 0,
    };

    let result = astar(
        &initial_state,
        |state| state.successors(),
        |state| state.minimum_distance(),
        |state| state.ready(),
    )
    .unwrap();

    result.1
}

pub fn solve(input: &str) -> Solution {
    let p1 = bring_up_equipment(input);
    // Part 2 is just 24 additional moves after bringing everything else to the top (p1) under the
    // assumption that something else also needs to be brought up from floor 1 (which is needed,
    // otherwise the elevator can't leave floor 1).
    let p2 = p1 + 24;

    Solution::new(p1, p2)
}
