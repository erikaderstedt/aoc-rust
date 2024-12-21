// https://adventofcode.com/2024/day/21

use std::str::from_utf8;

use pathfinding::prelude::bfs;

use crate::common::Solution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Numeric {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Directional {
    Left,
    Right,
    Up,
    Down,
    A,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    keyboard1: Numeric,         // "A" in keyboard2 puts state of keyboard 1 in typed
    keyboard2: Directional,     // radiated. Changes state of keyboard 1
    keyboard3: Directional,     // -40. Changes state of keyboard 2
}

impl State {
    fn start() -> State {
        State {
            keyboard1: Numeric::A,
            keyboard2: Directional::A,
            keyboard3: Directional::A,
        }
    }

    fn new_position_on_keyboard1(&self, button: &Directional) -> Option<Numeric> {
        match self.keyboard1 {
            Numeric::A => match button {
                Directional::Left => Some(Numeric::Zero),
                Directional::Right | Directional::Down => None,
                Directional::Up => Some(Numeric::Three),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Zero => match button {
                Directional::Right => Some(Numeric::A),
                Directional::Left | Directional::Down => None,
                Directional::Up => Some(Numeric::Two),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::One => match button {
                Directional::Right => Some(Numeric::Two),
                Directional::Left | Directional::Down => None,
                Directional::Up => Some(Numeric::Four),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Two => match button {
                Directional::Right => Some(Numeric::Three),
                Directional::Up => Some(Numeric::Five),
                Directional::Left => Some(Numeric::One),
                Directional::Down => Some(Numeric::Zero),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Three => match button {
                Directional::Right => None,
                Directional::Up => Some(Numeric::Six),
                Directional::Left => Some(Numeric::Two),
                Directional::Down => Some(Numeric::A),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Four => match button {
                Directional::Right => Some(Numeric::Five),
                Directional::Up => Some(Numeric::Seven),
                Directional::Left => None,
                Directional::Down => Some(Numeric::One),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Five => match button {
                Directional::Right => Some(Numeric::Six),
                Directional::Up => Some(Numeric::Eight),
                Directional::Left => Some(Numeric::Four),
                Directional::Down => Some(Numeric::Two),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Six => match button {
                Directional::Right => None,
                Directional::Up => Some(Numeric::Nine),
                Directional::Left => Some(Numeric::Five),
                Directional::Down => Some(Numeric::Three),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Seven => match button {
                Directional::Right => Some(Numeric::Eight),
                Directional::Up => None,
                Directional::Left => None,
                Directional::Down => Some(Numeric::Four),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Eight => match button {
                Directional::Right => Some(Numeric::Nine),
                Directional::Up => None,
                Directional::Left => Some(Numeric::Seven),
                Directional::Down => Some(Numeric::Five),
                Directional::A => Some(self.keyboard1.clone()),
            },
            Numeric::Nine => match button {
                Directional::Right => None,
                Directional::Up => None,
                Directional::Left => Some(Numeric::Eight),
                Directional::Down => Some(Numeric::Six),
                Directional::A => Some(self.keyboard1.clone()),
            },
        }
    }

    fn new_position_on_keyboard2(&self, button: &Directional) -> Option<Directional> {
        self.new_position_on_directional_keyboard(&self.keyboard2, button)
    }

    fn new_position_on_keyboard3(&self, button: &Directional) -> Option<Directional> {
        self.new_position_on_directional_keyboard(&self.keyboard3, button)
    }

    fn new_position_on_directional_keyboard(
        &self,
        current_position: &Directional,
        button: &Directional,
    ) -> Option<Directional> {
        match current_position {
            Directional::Left => match button {
                Directional::Left | Directional::Down | Directional::Up => None,
                Directional::Right => Some(Directional::Down),
                Directional::A => Some(current_position.clone()),
            },
            Directional::Right => match button {
                Directional::Left => Some(Directional::Down),
                Directional::Up => Some(Directional::A),
                Directional::Down | Directional::Right => None,
                Directional::A => Some(current_position.clone()),
            },
            Directional::Down => match button {
                Directional::Left => Some(Directional::Left),
                Directional::Right => Some(Directional::Right),
                Directional::Up => Some(Directional::Up),
                Directional::Down => None,
                Directional::A => Some(current_position.clone()),
            },
            Directional::Up => match button {
                Directional::Left | Directional::Up => None,
                Directional::Right => Some(Directional::A),
                Directional::Down => Some(Directional::Down),
                Directional::A => Some(current_position.clone()),
            },
            Directional::A => match button {
                Directional::Left => Some(Directional::Up),
                Directional::Down => Some(Directional::Right),
                Directional::Up | Directional::Right => None,
                Directional::A => Some(current_position.clone()),
            },
        }
    }

    fn apply(&self, button: &Directional) -> Option<State> {
        if *button == Directional::A {
            if self.keyboard3 == Directional::A {
                if self.keyboard2 == Directional::A {
                    Some(State {
                        keyboard1: self.keyboard1.clone(),
                        keyboard2: self.keyboard2.clone(),
                        keyboard3: self.keyboard3.clone(),
                    })
                    // panic!("This is handled elsewhere");
                } else {
                    if let Some(keyboard1) = self.new_position_on_keyboard1(&self.keyboard2) {
                        Some(State {
                            keyboard1,
                            keyboard2: self.keyboard2.clone(),
                            keyboard3: self.keyboard3.clone(),
                            // operator: self
                            //     .operator
                            //     .iter()
                            //     .cloned()
                            //     .chain(vec![button.clone()])
                            //     .collect(),
                        })
                    } else {
                        // The keyboard2 press resulted in an invalid keyboard 1 position
                        None
                    }
                }
            } else {
                if let Some(keyboard2) = self.new_position_on_keyboard2(&self.keyboard3) {
                    Some(State {
                        keyboard1: self.keyboard1.clone(),
                        keyboard2,
                        keyboard3: self.keyboard3.clone(),
                        // operator: self
                        //     .operator
                        //     .iter()
                        //     .cloned()
                        //     .chain(vec![button.clone()])
                        //     .collect(),
                    })
                } else {
                    None
                }
            }
        } else {
            if let Some(keyboard3) = self.new_position_on_keyboard3(button) {
                Some(State {
                    keyboard1: self.keyboard1.clone(),
                    keyboard2: self.keyboard2.clone(),
                    keyboard3,
                })
            } else {

                None
            }
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let keycodes: Vec<&str> = input.lines().collect();

    let p1 = keycodes
        .iter()
        .map(|keycode| -> usize {
            let complexity = from_utf8(&keycode.as_bytes()[0..3])
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let first_robot_types = Numeric::parse_string(&keycode);

            let mut start = State::start();
            // The end state is
            let mut length = 0;
            for what_to_type in first_robot_types.iter() {
                let path = bfs(
                    &start,
                    |state| {
                            [
                                Directional::Left,
                                Directional::A,
                                Directional::Right,
                                Directional::Down,
                                Directional::Up,
                            ]
                            .iter()
                            .filter_map(|d| state.apply(d))
                            .collect::<Vec<State>>()
                    },
                    |state| state.keyboard1 == *what_to_type && state.keyboard2 == Directional::A && state.keyboard3 == Directional::A,
                )
                .unwrap();

                length = length + path.len();
                start = State {
                    keyboard1: what_to_type.clone(),
                    keyboard2: Directional::A,
                    keyboard3: Directional::A,
                }
            }
            complexity * length
        })
        .sum::<usize>();
    let p2 = 0;

    Solution::new(p1, p2)
}

impl Numeric {
    fn parse_string(enter: &str) -> Vec<Numeric> {
        enter
            .as_bytes()
            .iter()
            .filter_map(|&u| -> Option<Numeric> {
                match u {
                    b'0' => Some(Numeric::Zero),
                    b'1' => Some(Numeric::One),
                    b'2' => Some(Numeric::Two),
                    b'3' => Some(Numeric::Three),
                    b'4' => Some(Numeric::Four),
                    b'5' => Some(Numeric::Five),
                    b'6' => Some(Numeric::Six),
                    b'7' => Some(Numeric::Seven),
                    b'8' => Some(Numeric::Eight),
                    b'9' => Some(Numeric::Nine),
                    b'A' => Some(Numeric::A),
                    _ => None,
                }
            })
            .collect()
    }
}
