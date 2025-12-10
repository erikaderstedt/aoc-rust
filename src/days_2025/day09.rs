// https://adventofcode.com/2025/day/9

use itertools::Itertools;
use crate::common::Solution;

// Note: it is possible to construct an input where this algorithm doesn't work - 
// it assumes that the largest rectangle formed by two points is inside the polygon.
// (vs being on the outside)

pub fn solve(input: &str) -> Solution {
    let red_tiles: Vec<RedTile> = input.lines().map(|line| {
        let (x, y) = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap();
        RedTile { x, y }
    }).collect();

    let rectangles: Vec<Rectangle> = 
        red_tiles.iter()
            .tuple_combinations()
            .map(|(a,b)| Rectangle { x0: a.x.min(b.x), y0: a.y.min(b.y), x1: a.x.max(b.x), y1: a.y.max(b.y) })
            .collect();

    let p1: i64 = rectangles.iter().map(|r| r.area()).max().unwrap();

    let mut p2 = 0;
    for r in rectangles.iter() {
        let a = r.area();
        if a > p2 && 
            // Check that no green line runs in the interior of r
            red_tiles.iter().circular_tuple_windows().all(|(p, q)|                    
                r.x0 >= p.x.max(q.x) || // Green line is to the left of r (or on its edge)
                r.x1 <= p.x.min(q.x) || // Green line is to the right of r (or on its edge)
                r.y0 >= p.y.max(q.y) || // Green line is above r (or on its edge)
                r.y1 <= p.y.min(q.y)) {   // Green line is below r (or on its edge)
            p2 = a;
        }
    }

    Solution::new(p1, p2)
}

struct RedTile {
    x: i64,
    y: i64
}

struct Rectangle {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64
}

impl Rectangle {
    fn area(&self) -> i64 {
        (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1)
    }
}