// https://adventofcode.com/2018/day/23

use crate::common::{parsed_from_each_line, Solution};
use itertools::Itertools;
use std::str::FromStr;

struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: u64,
}

pub fn solve(input: &str) -> Solution {
    let nanobots: Vec<Nanobot> = parsed_from_each_line(input);

    let best = nanobots.iter().max_by_key(|bot| bot.r).unwrap();

    let p1 = nanobots
        .iter()
        .filter(|bot| {
            bot.x.abs_diff(best.x) + bot.y.abs_diff(best.y) + bot.z.abs_diff(best.z) <= best.r
        })
        .count();

    let indices: Vec<(u64, i64)> = nanobots
        .iter()
        .map(|bot| {
            let d = (bot.x.abs() + bot.y.abs() + bot.z.abs()) as u64;
            vec![(d.checked_sub(bot.r).unwrap_or(0), 1), (d + bot.r, -1)].into_iter()
        })
        .flatten()
        .sorted_by_key(|s| s.0)
        .collect();

    // This works due to how my input is constructed. Seems to be typical.
    let mut highest_overlap = 0;
    let mut highest_overlap_at = 0;
    let mut overlap = 0;
    for index in indices.into_iter() {
        overlap = overlap + index.1;
        if overlap > highest_overlap {
            highest_overlap_at = index.0;
            highest_overlap = overlap;
        }
    }

    let p2 = highest_overlap_at;
    Solution::new(p1, p2)
}

impl FromStr for Nanobot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split('<')
            .skip(1)
            .next()
            .unwrap()
            .split('>')
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        let r = s.split('=').skip(2).next().unwrap().parse::<u64>().unwrap();
        Ok(Nanobot { x, y, z, r })
    }
}
