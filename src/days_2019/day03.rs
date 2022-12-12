use itertools::Itertools;

use crate::common::Solution;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
enum Direction {
    Up, Left,Down,Right,
}

#[derive(Debug)]
struct WireSegment {
    direction: Direction,
    length: u32,
}

fn all_traversed_coordinates(wire: &Vec<WireSegment>) -> Vec<(i32,i32)> {
    let mut coords: Vec<(i32,i32)> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for segment in wire.iter() {
        for _j in 0..segment.length {
            match segment.direction {
                Direction::Up => { y = y - 1; },
                Direction::Down => { y = y + 1; },
                Direction::Left => { x = x - 1; },
                Direction::Right => { x = x + 1; },
            }
            coords.push((x.clone(),y.clone()))
        }
    }
    coords
}

pub fn solve(input: &str) -> Solution {
    let lines: (&str,&str) = input.lines().take(2).collect_tuple().unwrap();
    let wire1: Vec<WireSegment> = lines.0.split(",").map(|x| WireSegment::from_str(x).expect("Bad wire segment")).collect();
    let wire2: Vec<WireSegment> = lines.1.split(",").map(|x| WireSegment::from_str(x).expect("Bad wire segment")).collect();

    let traversed_by1 = all_traversed_coordinates(&wire1);
    let traversed_by2 = all_traversed_coordinates(&wire2);

    let coords1: HashSet<(i32,i32)> = HashSet::from_iter(traversed_by1.iter().cloned());
    let coords2: HashSet<(i32,i32)> = HashSet::from_iter(traversed_by2.iter().cloned());
    let intersections: HashSet<(i32,i32)> = coords1.intersection(&coords2).cloned().collect();
    let nearest = intersections.iter().map(|(a,b)| a.abs() + b.abs()).min().unwrap();
    let fastest = intersections.iter().map(|p| -> usize {
        traversed_by1.iter().position(|x| p == x).unwrap() + 1 +
        traversed_by2.iter().position(|x| p == x).unwrap() + 1 
    }).min().unwrap();
    
    Solution::new(nearest, fastest)
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(format!("Bad direction {}", s)),
        }
    }
}

impl FromStr for WireSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            Err("String is too short".to_string())
        } else {
            let direction = Direction::from_str(&s.chars().next().unwrap().to_string())?;
            match s[1..].parse::<u32>() {
                Ok(length) => Ok(WireSegment { direction, length }),
                _ => Err("Unable to parse integer".to_string()),
            }
            
        }
    }
}