// https://adventofcode.com/2024/day/8

use crate::common::Solution;
use crate::grid::{Grid, GridElement};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Roof {
    Clear,
    Antenna(u8),
}

impl GridElement for Roof {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '.' => Some(Self::Clear),
            '0'..='9' | 'A'..='Z' | 'a'..='z' => Some(Self::Antenna(*c as u8)),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Clear => '.',
            Self::Antenna(t) => *t as char,
        }
    }
}

fn count_antipodes<const INCLUDE_ONLY_NEAREST: bool>(roof: &Grid<Roof>) -> usize {
    let sz = roof.rows * roof.cols;
    let max_step = roof.rows as isize;
    let mut antipodes = vec![false; sz];
    let try_range: Vec<isize> = if INCLUDE_ONLY_NEAREST {
        vec![2isize, -1isize]
    } else {
        (-max_step..max_step).collect()  
    };

    for i in ('0' as u8)..=('z' as u8) {
        let t = Roof::Antenna(i);
        let antennas: Vec<(isize, isize)> = roof
            .locations
            .iter()
            .enumerate()
            .filter(|(_, j)| **j == t)
            .map(|(l, _)| ((l / roof.cols) as isize, (l % roof.cols) as isize))
            .collect();

        for a in antennas.iter().combinations(2) {
            let (r1, c1) = a[0];
            let (r2, c2) = a[1];
            let dr = r2 - r1;
            let dc = c2 - c1;
            for u in try_range.iter().filter_map(|i| -> Option<usize> {
                let r = i * dr + r1;
                let c = i * dc + c1;
                if r >= 0 && c >= 0 && r < roof.rows as isize && c < roof.cols as isize {
                    Some((r as usize) * roof.cols + (c as usize))
                } else {
                    None
                }
            }) {
                antipodes[u] = true;
            }
        }
    }
    antipodes.into_iter().filter(|a| *a).count()
}

pub fn solve(input: &str) -> Solution {
    let roof: Grid<Roof> = Grid::load(input);

    let p1 = count_antipodes::<true>(&roof);
    let p2 = count_antipodes::<false>(&roof);

    Solution::new(p1, p2)
}
