// https://adventofcode.com/2018/day/13

use crate::common::Solution;
use crate::grid::{Direction, Grid, GridElement, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Rail {
    Empty,
    HorizontalTrack,
    VerticalTrack,
    TurnNW,
    TurnNE,
    Intersection,
    Cart(Direction),
}

impl GridElement for Rail {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '>' => Some(Rail::Cart(Direction::East)),
            '^' => Some(Rail::Cart(Direction::North)),
            'v' => Some(Rail::Cart(Direction::South)),
            '<' => Some(Rail::Cart(Direction::West)),
            '|' => Some(Rail::VerticalTrack),
            '-' => Some(Rail::HorizontalTrack),
            ' ' => Some(Rail::Empty),
            '+' => Some(Rail::Intersection),
            '/' => Some(Rail::TurnNE),
            '\\' => Some(Rail::TurnNW),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        panic!("Not implemented");
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Self::Left => Self::Straight,
            Self::Straight => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug)]
struct Cart {
    alive: bool,
    position: Position,
    direction: Direction,
    next_turn: Turn,
}

pub fn solve(input: &str) -> Solution {
    let rail_system = Grid::load(input);

    let mut carts: Vec<Cart> = rail_system
        .positions()
        .filter_map(|p| match rail_system.get(&p) {
            Some(Rail::Cart(direction)) => Some(Cart {
                alive: true,
                position: p,
                direction,
                next_turn: Turn::Left,
            }),
            _ => None,
        })
        .collect();

    let mut first_collision: Option<Position> = None;
    let mut tick = 0;
    let final_position = loop {
        if carts.iter().filter(|c| c.alive).count() == 1 {
            let cart = carts.iter().filter(|c| c.alive).next().unwrap();
            break cart.position.clone();
        }

        // Sort carts
        carts.sort_by_key(|cart| cart.position.row * rail_system.cols + cart.position.column);

        for i in 0..carts.len() {
            if !carts[i].alive {
                continue;
            }

            carts[i].position = carts[i].position.along(&carts[i].direction);

            match rail_system.get(&carts[i].position).unwrap() {
                Rail::TurnNE => {
                    carts[i].direction = match carts[i].direction {
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                        Direction::North => Direction::East,
                    };
                }
                Rail::TurnNW => {
                    carts[i].direction = match carts[i].direction {
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                        Direction::North => Direction::West,
                    };
                }
                Rail::Intersection => {
                    carts[i].direction = match carts[i].next_turn {
                        Turn::Left => carts[i].direction.counter_clockwise(),
                        Turn::Straight => carts[i].direction.clone(),
                        Turn::Right => carts[i].direction.clockwise(),
                    };
                    carts[i].next_turn = carts[i].next_turn.next();
                }
                _ => {}
            }

            // Check for collisions
            let p = carts[i].position.clone();
            if carts.iter().filter(|c| c.alive && c.position == p).count() == 2 {
                for cart in carts.iter_mut().filter(|c| c.position == p) {
                    cart.alive = false;
                }
                if first_collision == None {
                    first_collision = Some(p);
                }
            }
        }
        tick = tick + 1;
    };

    let p1 = format!("{:?}", first_collision.unwrap()).to_string();
    let p2 = format!("{:?}", final_position).to_string();
    Solution::new(p1, p2)
}
