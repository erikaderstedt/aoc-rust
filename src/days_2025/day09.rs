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

    let p1: i64 = red_tiles.iter().tuple_combinations()
        .map(|(a,b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .max().unwrap();
    let p2: i64 = red_tiles.iter().tuple_combinations()
        .filter(|(a,b)| {            
            red_tiles.iter().circular_tuple_windows()
                .all(|(p, q)|                    
                    // Check if the green line between p and q runs in the interior
                    // of the rectangle formed by (x0,y0) and (x1,y1)
                    a.x.min(b.x) >= p.x.max(q.x) || a.x.max(b.x) <= p.x.min(q.x) ||
                    a.y.min(b.y) >= p.y.max(q.y) || a.y.max(b.y) <= p.y.min(q.y))
        })
        .map(|(a,b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .max().unwrap();

    Solution::new(p1, p2)
}
