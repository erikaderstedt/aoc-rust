// https://adventofcode.com/2025/day/9

use itertools::Itertools;
use crate::common::Solution;

struct RedTile {
    x: i64,
    y: i64
}

pub fn solve(input: &str) -> Solution {

    let red_tiles: Vec<RedTile> = input.lines().map(|line| {
        let v: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
        RedTile { x: v[0], y: v[1] }
    }).collect();

    let p1: i64 = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a,b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .max().unwrap();
    let p2: i64 = red_tiles
        .iter()
        .tuple_combinations()
        .filter(|(a,b)| {
            let x0 = a.x.min(b.x);
            let x1 = a.x.max(b.x);
            let y0 = a.y.min(b.y);
            let y1 = a.y.max(b.y);
            !red_tiles.iter().circular_tuple_windows()
                .any(|(p, q)|
                    // Check if either of our corners is inside their rect
                    x0 < p.x.max(q.x) &&
                    x1 > p.x.min(q.x) &&
                    y0 < p.y.max(q.y) &&
                    y1 > p.y.min(q.y)
            )
        })
        .map(|(a,b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .max().unwrap();

    Solution::new(p1, p2)
}
