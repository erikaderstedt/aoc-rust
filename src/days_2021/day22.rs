// https://adventofcode.com/2021/day/22
use crate::common::Solution;
use itertools::Itertools;
use std::str::FromStr;
use crate::common::parsed_from_each_line;

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum FlipState {
    On,
    Off,
}

impl FlipState {
    fn inverted(&self) -> FlipState {
        match self {
            FlipState::Off => FlipState::On,
            FlipState::On => FlipState::Off,
        }
    }
}

const DIMS: usize = 3;

#[derive(Debug,Copy,Clone)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn inclusive_length(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        if other.max < self.min || other.min > self.max {
            None
        } else if self.min >= other.min && self.min <= other.max {
            Some(Range { min: self.min, max: isize::min(other.max, self.max) })
        } else if self.max >= other.min && self.max <= other.max {
            Some(Range { min: isize::max(other.min, self.min), max: self.max })
        } else {
            Some(other.clone())
        }
    }
}

#[derive(Debug,Copy,Clone)]
struct Cuboid {
    flip: FlipState,
    ranges: [Range;DIMS]
}

impl Cuboid {

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let ranges: Vec<Range> = self.ranges.iter()
                        .zip(other.ranges.iter())
                        .filter_map(|(r1, r2)| r1.intersection(r2))
                        .collect();
        if ranges.len() == 3 {
            Some(Cuboid { flip: other.flip.inverted(), ranges: [ranges[0], ranges[1], ranges[2]] })
        } else {
            None
        }
    }

    fn volume(&self) -> usize {
        self.ranges[0].inclusive_length() *
        self.ranges[1].inclusive_length() *
        self.ranges[2].inclusive_length()
    }
}

fn reactor_on_count<'a,I>(cuboids: I) -> usize 
    where I: Iterator<Item=&'a Cuboid> {

    let mut painted: Vec<Cuboid> = Vec::new();

    for cuboid in cuboids {
        let mut added: Vec<Cuboid> = painted.iter()
            .filter_map(|p| cuboid.intersection(p))
            .collect();

        painted.append(&mut added);

        if cuboid.flip == FlipState::On {
            painted.push(cuboid.clone());
        }
    }

    painted.iter().fold(0, |s, c|
        if c.flip == FlipState::On { s + c.volume() } else { s - c.volume() }
    )
}

pub fn solve(input: &str) -> Solution {
    let cuboids: Vec<Cuboid> = parsed_from_each_line(input);
    
    let m1 = reactor_on_count(cuboids.iter().take(20));
    let m2 = m1 + reactor_on_count(cuboids.iter().skip(20));
    
    Solution::new(m1,m2)
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("..") {
            Some((min,max)) => {
                let min = min.parse::<isize>().map_err(|_| "Invalid integer literal")?;
                let max = max.parse::<isize>().map_err(|_| "Invalid integer literal")?;
                Ok(Range { min, max })
            },
            _ => Err("Malformed line."),
        }
    }
}

impl FromStr for Cuboid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((on_or_off, ranges)) => {
                let flip = if on_or_off == "on" { FlipState::On } else { FlipState::Off };
                let (x,y,z) = ranges.split(',').map(|r| r[2..].parse::<Range>().unwrap()).collect_tuple().unwrap();
                Ok(Cuboid { flip, ranges: [x,y,z] }) },
            _ => Err("Malformed line."),
        }
    }
}
