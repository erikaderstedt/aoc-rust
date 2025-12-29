use crate::common::Solution;
use itertools::Itertools;

pub fn solve(input: &str) -> Solution {
    let locations: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split(", ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let width = locations.iter().map(|l| l.0).max().unwrap();
    let height = locations.iter().map(|l| l.1).max().unwrap();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut grid: Vec<usize> = vec![usize::MAX; width * height];
    for x in 0..width {
        for y in 0..height {
            let closest = locations
                .iter()
                .enumerate()
                .map(|(i, l)| (i, x.abs_diff(l.0) + y.abs_diff(l.1)))
                .min_by_key(|l| l.1)
                .unwrap();
            let n_closest = locations
                .iter()
                .filter(|l| x.abs_diff(l.0) + y.abs_diff(l.1) == closest.1)
                .count();
            if n_closest == 1 {
                grid[x + (width * y)] = closest.0;
            }

            let distance_to_all: usize = locations
                .iter()
                .map(|l| x.abs_diff(l.0) + y.abs_diff(l.1))
                .sum();
            if distance_to_all < 10000 {
                p2 = p2 + 1;
            }
        }
    }
    let w = width as usize;
    let h = height as usize;
    for i in 0..locations.len() {
        let area = grid.iter().filter(|g| **g == i).count();
        if area <= p1 {
            continue;
        }
        if !grid
            .iter()
            .take(w)
            .chain(grid.iter().skip(w * (h - 1)))
            .chain(grid.iter().skip(w - 1).step_by(w))
            .chain(grid.iter().step_by(w))
            .any(|p| *p == i)
        {
            p1 = area;
        }
    }

    Solution::new(p1, p2)
}
