// https://adventofcode.com/2023/day/18

use crate::{common::Solution, grid::Direction};

struct Item {
    direction: Direction,
    length: i64,
}

fn part<F>(input: &str, f: F) -> i64 where F: Fn(&str) -> Item {
    input.lines().map(f).scan((0i64, 1i64), |state, item| {
        match item.direction {
            // Dig a single line
            Direction::East => { state.0 += item.length; state.1 += item.length; },
            // Digging the same way back does not add any area.
            Direction::West => { state.0 -= item.length; },
            // Digging south adds the area under the line, plus a single line
            Direction::South => { state.1 += item.length * state.0 + item.length; },
            // Digging north subtracts the area under the line, but not the single line.
            Direction::North => { state.1 -= item.length * state.0},
        };
        Some(state.1)
    }).last().unwrap().abs()  // Counter-clockwise loops give a negative area
}

pub fn solve(input: &str) -> Solution {
    let p1 = part(input, |line| {
        let direction = match line.as_bytes()[0] {
            b'U' => Direction::North,
            b'D' => Direction::South,
            b'L' => Direction::West,
            b'R' => Direction::East,
            _ => panic!("Unknown direction."),
        };
        let d1 = line.as_bytes()[2] as i64 - '0' as i64;
        let d2 = line.as_bytes()[3] as i64 - '0' as i64;
        let length = if d2 < 0 { d1 } else { d1 * 10 + d2 };
        Item { direction, length }
    });
    
    let p2 = part(input, |line| {
        let (_, hex) = line.split_once('#').unwrap();
        let length = i64::from_str_radix(&hex[0..5], 16).unwrap();
        let direction = match hex.as_bytes()[5] {
            b'3' => Direction::North,
            b'1' => Direction::South,
            b'2' => Direction::West,
            b'0' => Direction::East,
            _ => panic!("Unknown direction."),
        };
        Item { direction, length }
    });

    Solution::new(p1, p2)
}
