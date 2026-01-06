// https://adventofcode.com/2017/day/19

use crate::{
    common::Solution,
    grid::{Direction, Grid, GridElement, Position},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Network {
    HorizontalRail,
    VerticalRail,
    Turn,
    Signpost(char),
    Empty,
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<Network> = Grid::load(input);

    let start_index = grid
        .locations
        .iter()
        .position(|p| *p == Network::VerticalRail)
        .unwrap();

    let mut direction = Direction::South;
    let mut position = Position {
        row: 0,
        column: start_index,
    };

    let mut p1_chars = vec![];
    let mut p2 = 0;
    let mut passed_sign = false;
    loop {
        p2 += 1;
        // Continue along the same direction until a '+' is found.
        // If we just passed a signpost an empty spot will be the end.
        position = position.along(&direction);
        match grid.get(&position) {
            Some(Network::VerticalRail) | Some(Network::HorizontalRail) => {}
            Some(Network::Signpost(x)) => {
                passed_sign = true;
                p1_chars.push(x.clone());
                continue;
            }
            Some(Network::Turn) => {
                direction = match direction {
                    Direction::East | Direction::West => match (
                        grid.get(&position.along(&Direction::North)),
                        grid.get(&position.along(&Direction::South)),
                    ) {
                        (Some(n), _) if n == Network::VerticalRail => Direction::North,
                        (_, Some(s)) if s == Network::VerticalRail => Direction::South,
                        _ => panic!(
                            "Turn but no matching track at {:?} coming from {:?}!",
                            position, direction
                        ),
                    },
                    Direction::North | Direction::South => match (
                        grid.get(&position.along(&Direction::West)),
                        grid.get(&position.along(&Direction::East)),
                    ) {
                        (Some(w), _) if w == Network::HorizontalRail => Direction::West,
                        (_, Some(e)) if e == Network::HorizontalRail => Direction::East,
                        _ => panic!(
                            "Turn but no matching track at {:?} coming from {:?}!",
                            position, direction
                        ),
                    },
                };
            }
            Some(Network::Empty) => {
                if passed_sign {
                    break;
                }
            }
            None => {
                break;
            }
        };
        passed_sign = false;
    }

    let p1: String = p1_chars.into_iter().collect();

    Solution::new(p1, p2)
}

impl GridElement for Network {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            ' ' => Some(Self::Empty),
            '+' => Some(Self::Turn),
            '|' => Some(Self::VerticalRail),
            '-' => Some(Self::HorizontalRail),
            'A'..='Z' => Some(Self::Signpost(c.clone())),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Turn => '+',
            Self::HorizontalRail => '-',
            Self::VerticalRail => '|',
            Self::Signpost(c) => c.clone(),
            Self::Empty => ' ',
        }
    }
}
