// https://adventofcode.com/2022/day/10

use crate::common::{Solution};
use itertools::Itertools;
use std::hash::Hash;
use pathfinding::prelude::bfs;
// use std::collections::HashMap;
use std::fmt;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;
const LOWEST: u8 = 'a' as u8;
const HIGHEST: u8 = 'z' as u8;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Pos {
    fn from(x: usize, y: usize) -> Pos { Pos { x, y }}
}

fn find_path<FS>(dtm: &Vec<Vec<u8>>, start: &Pos, end: FS, climbing: bool) -> Option<usize>
where
    FS: FnMut(&Pos) -> bool,
{
    let width = dtm[0].len();
    let height = dtm.len();

    bfs(start, |p| {
        let mut v: Vec<Pos> = Vec::with_capacity(4);
        let compare = match dtm[p.y][p.x] {
            START => LOWEST,
            END => HIGHEST,
            i => i,
        };
        if p.x > 0 && ((climbing && dtm[p.y][p.x-1] <= compare + 1) || (!climbing && dtm[p.y][p.x-1] >= compare - 1)) {
            v.push(Pos { x: p.x - 1, y: p.y})
        }
        if p.x < width - 1 && ((climbing && dtm[p.y][p.x+1] <= compare + 1) || (!climbing && dtm[p.y][p.x+1] >= compare - 1)) {
            v.push(Pos { x: p.x + 1, y: p.y})
        }
        if p.y > 0 && ((climbing && dtm[p.y-1][p.x] <= compare + 1) || (!climbing && dtm[p.y-1][p.x] >= compare - 1)) {
            v.push(Pos { x: p.x, y: p.y - 1})
        }
        if p.y < height - 1 && ((climbing && dtm[p.y+1][p.x] <= compare + 1) || (!climbing && dtm[p.y+1][p.x] >= compare - 1)) {
            v.push(Pos { x: p.x, y: p.y + 1})
        }
        v
    }, end).map(|v| v.len() - 1)
}

pub fn solve(input: &str) -> Solution {
    let dtm: Vec<Vec<u8>> = input.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let x = dtm.iter().find_map(|line| line.iter().find_position(|&&c| c == START)).unwrap().0;
    let y = dtm.iter().find_position(|line| line.contains(&START)).unwrap().0;
    let target_x = dtm.iter().find_map(|line| line.iter().find_position(|&&c| c == END)).unwrap().0;
    let target_y = dtm.iter().find_position(|line| line.contains(&END)).unwrap().0;

    let starting_pos = Pos::from(x, y);            
    let goal_pos = Pos { x: target_x, y: target_y };
    let p1 = find_path(&dtm, &starting_pos, |p| *p == goal_pos, true).expect("Unable to reach E");
    let p2 = find_path(&dtm, &goal_pos, |p| 
        match dtm[p.y][p.x] {
            START | LOWEST => true,
            _ => false,
        }, false).expect("Unable to reach any 'a'.");
    
    Solution::new(p1,p2)
}
