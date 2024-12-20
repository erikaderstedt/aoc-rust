// https://adventofcode.com/2024/day/20

use pathfinding::prelude::bfs;
use crate::{common::Solution, grid::{Grid, GridElement, Position}};

#[derive(Debug, PartialEq,Eq,Clone)]
enum MapElement {
    Empty,
    Wall,
    Start,
    End,
}

fn num_fast_enough_cheats<const CHEAT_DISTANCE: usize>(original: &Vec<Position>) -> usize {
    let mut r = 0;
    for (i1, p1) in original.iter().enumerate() {
        for (i2, p2) in original.iter().enumerate().skip(i1 + 1) {
            let d = p1.manhattan_distance(p2);
            if d <= CHEAT_DISTANCE && (i2 - i1) - d >= 100 {
                r = r + 1;
            }
        }
    }
    r
}

pub fn solve(input: &str) -> Solution {
    let map: Grid<MapElement> = Grid::load(input);

    let start = map.find(&MapElement::Start).unwrap();
    let end = map.find(&MapElement::End).unwrap();

    // No forks - pathfinding not needed so this can probably be reimplemented with
    // a simpler algoritm and removed.
    let original = bfs(&start, |p| map.neighbor_positions_satisfying_condition(p, |_, m| *m != MapElement::Wall), |p| *p == end).unwrap();

    let p1 = num_fast_enough_cheats::<2>(&original);
    let p2 = num_fast_enough_cheats::<20>(&original);

    Solution::new(p1, p2)
}

impl GridElement for MapElement {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '.' => Some(MapElement::Empty),
            '#' => Some(MapElement::Wall),
            'S' => Some(MapElement::Start),
            'E' => Some(MapElement::End),

            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Start => 'S',
            Self::End => 'E',
        }
    }
}