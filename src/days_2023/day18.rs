// https://adventofcode.com/2023/day/17

use crate::{common::Solution, grid::Direction};

#[derive(Debug)]
struct Item {
    direction: Direction,
    length: usize,
}

fn part<F>(input: &str, f: F) -> i64 where F: Fn(&str) -> Item {
    let mut x = 0i64;
    let mut y = 0i64;
    let mut area = 0;
    for line in input.lines() {        
        let item = f(line);
        let prev = (x,y);
        match item.direction {
            Direction::East => { x += item.length as i64; },
            Direction::South => { y += item.length as i64; },
            Direction::North => { y -= item.length as i64; },
            Direction::West => { x -= item.length as i64; },
        }
        area += (y - prev.1) * (x + prev.0) + item.length as i64;
    }
    area/2 + 1
}

pub fn solve(input: &str) -> Solution {
    let p1 = part(input, |line| {
        let direction = match line.as_bytes()[0] {
            b'U' => Direction::North,
            b'D' => Direction::South,
            b'L' => Direction::West,
            b'R' => Direction::East,
            _ => panic!("?"),
        };
        let length = line.split(' ').skip(1).next().unwrap().parse::<usize>().unwrap();
        Item { direction, length }
    });
    
    let p2 = part(input, |line| {
        let hex = line.split('#').skip(1).next().unwrap();
        let v = i64::from_str_radix(&hex[0..6], 16).unwrap();
        let length = v / 16;
        let direction = match v % 16 {
            3 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            0 => Direction::East,
            _ => panic!("?"),
        };
        Item { direction, length: length as usize }
    });

    Solution::new(p1, p2)
}
