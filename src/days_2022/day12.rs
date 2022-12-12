// https://adventofcode.com/2022/day/10

use crate::common::{Solution};
use pathfinding::prelude::bfs;
use crate::grid::{Grid,GridElement};

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;
const LOWEST: u8 = 'a' as u8;
const HIGHEST: u8 = 'z' as u8;

impl GridElement for u8 {
    fn from_char(c: &char) -> Option<Self> { Some(*c as u8) }
    fn to_char(&self) -> char { *self as char }
}

pub fn solve(input: &str) -> Solution {
    let dtm: Grid<u8> = Grid::load(input);
    let start = dtm.find(&START).expect("Unable to find starting position");
    let end = dtm.find(&END).expect("Unable to find ending position");

    let p1 = bfs(&start, |p| 
        dtm.neighbor_positions_satisfying_condition(p, |current, n| {
            let compare: u8 = if *current == START { LOWEST } else { *current };
            *n <= compare + 1 })
        , |p| dtm[p] == END).map(|v| v.len() - 1).expect("Unable to reach end position");
    let p2 = bfs(&end, |p| 
        dtm.neighbor_positions_satisfying_condition(p, |current, n| {
            let compare: u8 = if *current == END { HIGHEST } else { *current };
            *n >= compare - 1 })
        , |p| match dtm[p] { START | LOWEST => true, _ => false }).map(|v| v.len() - 1).expect("Unable to reach end position");

    Solution::new(p1,p2)
}
