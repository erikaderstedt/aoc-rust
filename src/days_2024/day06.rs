// https://adventofcode.com/2024/day/6

use crate::common::Solution;
use crate::grid::{Direction, Grid, GridElement};

pub fn solve(input: &str) -> Solution {
    let floor: Grid<Floor> = Grid::load(input).enclosed(Floor::OutOfBounds);
    let sz = floor.rows * floor.cols;

    let mut pos = floor.find(&Floor::Guard).unwrap();
    let mut possible_obstacles = vec![false; sz];
    let mut visited = vec![false; sz];
    let mut facing = Direction::North;

    loop {
        visited[pos.row * floor.cols + pos.column] = true;

        let next_pos = pos.along(&facing);
        let next_pos_index = next_pos.row * floor.cols + next_pos.column;
        match floor.get(&next_pos) {
            Some(Floor::OutOfBounds) => break,
            Some(Floor::Obstruction) => facing = facing.clockwise(),
            Some(Floor::Clear) | Some(Floor::Guard) => {
                if !possible_obstacles[next_pos_index] && 
                    !visited[next_pos_index]  {
                    let mut dir = facing.clone();
                    let mut p = pos.clone();
                    let mut visited_during_loop_test = vec![0u8; sz];
                    loop {
                        if (visited_during_loop_test[p.row * floor.cols + p.column] & dir.u8()) != 0
                        {
                            possible_obstacles[next_pos_index] = true;
                            break;
                        }
                        visited_during_loop_test[p.row * floor.cols + p.column] |= dir.u8();
                        let new_p = p.along(&dir);
                        if new_p == next_pos {
                            // We are pretending that next_pos is an obstacle
                            dir = dir.clockwise()
                        } else {
                            match floor.get(&new_p).unwrap() {
                                Floor::OutOfBounds => break,
                                Floor::Guard | Floor::Clear => p = new_p,
                                Floor::Obstruction => dir = dir.clockwise(),
                            }
                        }
                    }
                }
                pos = next_pos;
            }
            None => panic!("What?"),
        }
    }

    let p1 = visited.into_iter().filter(|v| *v).count();
    let p2 = possible_obstacles.into_iter().filter(|v| *v).count();

    Solution::new(p1, p2)
}

#[derive(PartialEq, Eq, Clone)]
enum Floor {
    Clear,
    Obstruction,
    Guard,
    OutOfBounds,
}

impl GridElement for Floor {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '.' => Some(Self::Clear),
            '#' => Some(Self::Obstruction),
            '^' => Some(Self::Guard),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Clear => '.',
            Self::Obstruction => '#',
            Self::Guard => '@',
            Self::OutOfBounds => 'X',
        }
    }
}