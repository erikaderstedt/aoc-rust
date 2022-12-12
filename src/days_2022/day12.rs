// https://adventofcode.com/2022/day/10

use crate::common::{Solution};
use itertools::Itertools;
use std::hash::Hash;
use pathfinding::prelude::bfs;
// use std::collections::HashMap;
use std::fmt;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;


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

fn find_path<FS>(dtm: &Vec<Vec<u8>>, start: &Pos, end: FS) -> Option<usize>
where
    FS: FnMut(&Pos) -> bool,
{
    let width = dtm[0].len();
    let height = dtm.len();

    bfs(start, |p| {
        let mut v: Vec<Pos> = Vec::with_capacity(4);
        let compare = match dtm[p.y][p.x] {
            START => 'a' as u8,
            END => 'z' as u8,
            i => i,
        } + 1;
        if p.x > 0 && dtm[p.y][p.x-1] <= compare {
            v.push(Pos { x: p.x - 1, y: p.y})
        }
        if p.x < width - 1 && dtm[p.y][p.x+1] <= compare {
            v.push(Pos { x: p.x + 1, y: p.y})
        }
        if p.y > 0 && dtm[p.y-1][p.x] <= compare {
            v.push(Pos { x: p.x, y: p.y - 1})
        }
        if p.y < height - 1 && dtm[p.y+1][p.x] <= compare {
            v.push(Pos { x: p.x, y: p.y + 1})
        }
        v
    }, end).map(|v| v.len() - 1)
}

pub fn solve(input: &str) -> Solution {
    let dtm: Vec<Vec<u8>> = input.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let x = dtm.iter().find_map(|line| line.iter().find_position(|&&c| c == 'S' as u8)).unwrap().0;
    let y = dtm.iter().find_position(|line| line.contains(&('S' as u8))).unwrap().0;
    let target_x = dtm.iter().find_map(|line| line.iter().find_position(|&&c| c == 'E' as u8)).unwrap().0;
    let target_y = dtm.iter().find_position(|line| line.contains(&('E' as u8))).unwrap().0;

    // // Pre-build a hashmap of Position -> vec![Position]
    // let m: HashMap<Pos, Vec<Pos>> = (0..height)
    //     .zip(0..width)
    //     .map(|(y,x)| {
    //         let mut v: Vec<Pos> = Vec::with_capacity(4);
    //         if x > 0 && dtm[y][x-1] <= dtm[y][x] { v.push(Pos::from(x-1, y)) }
    //         if x < width - 1  && dtm[y][x+1] <= dtm[y][x] { v.push(Pos::from(x+1, y)) }
    //         if y > 0 && dtm[y-1][x] <= dtm[y][x] { v.push(Pos::from(x, y-1)) }
    //         if y < height - 1 && dtm[y+1][x] <= dtm[y][x] { v.push(Pos::from(x, y+1)) }
    //         (Pos::from(x, y), v)
    //     })
    //     .collect();

    let starting_pos = Pos::from(x, y);            
    let goal_pos = Pos { x: target_x, y: target_y };
    let p1 = find_path(&dtm, &starting_pos, |p| *p == goal_pos).expect("Unable to reach E");
    let p2 = find_path(&dtm, &goal_pos, |p| dtm[p.y][p.x] == ('a' as u8) || dtm[p.y][p.x] == START).expect("Unable to reach any 'a'.");
    


    Solution::new(p1,p2)
}
