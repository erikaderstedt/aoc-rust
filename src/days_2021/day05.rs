// https://adventofcode.com/2021/day/5
use crate::common::Solution;
use std::str::FromStr;
use itertools::Itertools;
use crate::common::parsed_from_each_line;

struct Vent {
    from_x: usize,  from_y: usize,
    to_x: usize,    to_y: usize,
    dx: i32,        dy: i32,
}

struct VentIterator<'a> {
    vent: &'a Vent,
    x: usize,
    y: usize,
    reached_end: bool
}

impl FromStr for Vent {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" -> ") {            
            Some((from, to)) => {
                let (from_x, from_y) = from.split(',').map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap();
                let (to_x, to_y) = to.split(',').map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap();
                let dx = if from_x == to_x { 0 } else { if from_x < to_x { 1 } else { -1 } };
                let dy = if from_y == to_y { 0 } else { if from_y < to_y { 1 } else { -1 } };
                Ok(Vent { from_x, from_y, to_x, to_y, dx, dy })
                },
            _ => Err("Malformed line."),
        }
    }
}

impl<'a> IntoIterator for &'a Vent {
    type Item = (usize, usize);
    type IntoIter = VentIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        VentIterator {
            vent: self,
            x: self.from_x,
            y: self.from_y,
            reached_end: false,
        }
    }
}

impl<'a> Iterator for VentIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.reached_end { None }
        else if self.x == self.vent.to_x && self.y == self.vent.to_y {
            self.reached_end = true;
            Some((self.x, self.y))
        } else {
            let r = (self.x, self.y);
            self.x = ((self.x as i32) + self.vent.dx) as usize;
            self.y = ((self.y as i32) + self.vent.dy) as usize;
            Some(r)
        }
    }
}

const GRID_SIZE: usize = 1000;

fn check_overlaps(vents: &Vec<Vent>, use_diagonals: bool) -> usize {
    let mut grid = [[0u8;GRID_SIZE];GRID_SIZE];
    for vent in vents.iter() {
        if use_diagonals || (vent.dx == 0 || vent.dy == 0) {
            for (x, y) in vent { grid[y][x] += 1 }
        }
    }
    grid.iter().fold(0usize, |s, row| s + row.iter().filter(|v| **v > 1).count())
}

pub fn solve(input: &str) -> Solution {
    let vents: Vec<Vent> = parsed_from_each_line(input);

    let m1 = check_overlaps(&vents, false);
    let m2 = check_overlaps(&vents, true);

    Solution::new(m1, m2)
}