// https://adventofcode.com/2016/day/1

use crate::common::Solution;
use crate::grid::Direction;

pub fn solve(input: &str) -> Solution {
    let mut visited = vec![(0, 0)];
    let mut direction = Direction::North;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut p2: Option<i64> = None;
    for m in input.trim().split(", ") {
        let (d, l) = m.split_at(1);
        let l = l.parse::<i64>().unwrap();

        direction = match d {
            "L" => direction.counter_clockwise(),
            "R" => direction.clockwise(),
            c => panic!("Unknown direction '{}'", c),
        };

        for _ in 0..l {
            match direction {
                Direction::East => x += 1,
                Direction::West => x -= 1,
                Direction::North => y += 1,
                Direction::South => y -= 1,
            }
            if visited.contains(&(x, y)) && p2.is_none() {
                p2 = Some(x.abs() + y.abs());
            }

            visited.push((x, y));
        }
    }

    let p1 = x.abs() + y.abs();
    let p2 = p2.unwrap_or(0);

    Solution::new(p1, p2)
}
