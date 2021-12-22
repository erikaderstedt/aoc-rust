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

const DIMS: usize = 3;

#[derive(Debug,Copy,Clone)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn inclusive_range(&self) -> usize {
        (self.max - self.min + 1) as usize
    }

    fn intersects(&self, other: &Range) -> bool {
        (self.min >= other.min && self.min <= other.max) ||
        (self.max >= other.min && self.max <= other.max)
    }

    fn inside(&self, other: &Range) -> bool {
        (self.min >= other.min && self.min <= other.max) &&
        (self.max >= other.min && self.max <= other.max)
    }
}

#[derive(Debug,Copy,Clone)]
struct Cuboid {
    flip: FlipState,
    ranges: [Range;DIMS]
}

impl Cuboid {

    fn volume(&self) -> usize {
        self.ranges[0].inclusive_range() *
        self.ranges[1].inclusive_range() *
        self.ranges[2].inclusive_range()
    }

    // fn touches(&self, x: isize, y: isize, z: isize) -> bool { 
    //     self.x_min <= x && self.x_max >= x &&
    //     self.y_min <= y && self.y_max >= y &&
    //     self.z_min <= z && self.z_max >= z
    // }

    fn intersects(&self, other: &Cuboid) -> bool {
        self.ranges[0].intersects(&other.ranges[0]) ||
        self.ranges[1].intersects(&other.ranges[1]) ||
        self.ranges[2].intersects(&other.ranges[2])
    }

    fn encompassed_by(&self, other: &Cuboid) -> bool {
        self.ranges[0].inside(&other.ranges[0]) &&
        self.ranges[1].inside(&other.ranges[1]) &&
        self.ranges[2].inside(&other.ranges[2])
    }

    /// A split at a certain x gives 
    fn split(&self, dimension: usize, i: isize) -> (Cuboid, Cuboid) {
        let flip = self.flip;
        match dimension {
            0 => (Cuboid { flip, ranges: [Range { min: self.ranges[0].min, max: i }, self.ranges[1], self.ranges[2]] },
                  Cuboid { flip, ranges: [Range { min: i+1, max: self.ranges[0].max}, self.ranges[1], self.ranges[2]] }),
            1 => (Cuboid { flip, ranges: [self.ranges[0], Range { min: self.ranges[1].min, max: i }, self.ranges[2]] },
                  Cuboid { flip, ranges: [self.ranges[0], Range { min: i+1, max: self.ranges[1].max}, self.ranges[2]] }),
            2 => (Cuboid { flip, ranges: [self.ranges[0], self.ranges[1], Range { min: self.ranges[2].min, max: i }] },
                  Cuboid { flip, ranges: [self.ranges[0], self.ranges[1], Range { min: i+1, max: self.ranges[2].max}] }),
            _ => unreachable!("")
        }
    }

    /// Cuts self up in pieces
    fn cleave(&self, other: &Cuboid) -> Vec<Cuboid> {
        // Find a direction where self extends outside other.
        println!("=== Cleave ===");
        println!(" self: {:?}", self);
        println!("other: {:?}", other);
        // Split in that direction.
        let mut v = Vec::new();
        let mut chopped_up: Cuboid = self.clone();
        for dim in 0..DIMS {
            if chopped_up.ranges[dim].min < other.ranges[dim].min {
                let (external, internal) = chopped_up.split(dim, other.ranges[dim].min - 1);
                println!("Cutting off dimension {}: {:?}", dim, external);
                println!("Remains {:?}", internal);
                v.push(external);
                chopped_up = internal;
                }
            if chopped_up.ranges[dim].max > other.ranges[dim].max {
                let (internal,external) = chopped_up.split(dim, other.ranges[dim].max);
                println!("Cutting off dimension {}: {:?}", dim, external);
                println!("Remains {:?}", internal);
                v.push(external);
                chopped_up = internal;
            }
        }
        println!("=== Done ===");
        v
    } 
}



pub fn solve(input: &str) -> Solution {
    

    let mut cuboids: Vec<Cuboid> = parsed_from_each_line(input);
    cuboids.reverse();

    // Start with cuboid 0.
    // Go to cuboid 1. Go through previous
    // cuboids, either keeping them if there is no intersection,
    // or cleaving off the extra parts of the previous cuboid that do not 
    // intersect with the current cuboid. 
    // If the previous is encompassed by the current, just remove it.

    // TODO: start from the back of v. Also, if we are encompassed by another,
    // just copy the remainin 
    let mut v: Vec<Cuboid> = Vec::new();
    while let Some(mut next_cuboid) = cuboids.pop() {
        println!("Now handling {:?}", next_cuboid);
        let mut v2: Vec<Cuboid> = Vec::new();
        for existing in v.into_iter() {
            if existing.encompassed_by(&next_cuboid) {
                println!("{:?} is encompassed.", existing);
                continue; }
            if existing.intersects(&next_cuboid) {
                println!("{:?} intersects", existing);
                v2.extend(existing.cleave(&next_cuboid));                
            } else {
                println!("{:?} does not intersect", existing);
                v2.push(existing);
            }
        }
        v2.push(next_cuboid);
        v = v2;
    }

    for c in v.iter() {
        println!("{:?}", c);
    }

    let m1: usize = v.into_iter().map(|c| if c.flip == FlipState::On { c.volume() } else { 0 }).sum();



    let m2 = 0;
    // const SIZE: usize = 102;
    // const HSIZE: isize = 51;

    // let m1 = {
    //     let mut grid = [[[0u8;SIZE];SIZE];SIZE];
    //     for c in cuboids.iter().take(20) {
    //         let mut j = 0;
    //         let v = match c.flip { FlipState::On => 1, FlipState::Off => 0 };
    //         for z in c.z_min..=c.z_max {
    //             for y in c.y_min..=c.y_max {
    //                 for x in c.x_min..=c.x_max {
    //                     j += 1;
    //                     grid[(z+HSIZE) as usize][(y+HSIZE) as usize][(x+HSIZE) as usize] = v;
    //                 }
    //             }
    //         }
    //     }
    //     let mut i: usize = 0;
    //     for z in 0..SIZE {
    //         for y in 0..SIZE {
    //             for x in 0..SIZE {
    //                 i += grid[z][y][x] as usize;
    //             }
    //         }
    //     }
    //     i
    // };

    // let x_min = cuboids.iter().map(|c| c.x_min).min().unwrap();
    // let x_max = cuboids.iter().map(|c| c.x_max).max().unwrap();
    // let y_min = cuboids.iter().map(|c| c.y_min).min().unwrap();
    // let y_max = cuboids.iter().map(|c| c.y_max).max().unwrap();
    // let z_min = cuboids.iter().map(|c| c.z_min).min().unwrap();
    // let z_max = cuboids.iter().map(|c| c.z_max).max().unwrap();
    // println!("{} {} {} {} {} {}", x_min, x_max, y_min, y_max, z_min, z_max);

    // for c in cuboids.iter() {
    //     println!("{} -> {}, {} -> {}, {} -> {}", 
    //     x_min-c.x_min,
    //     x_max-c.x_min,
    //     y_min-c.y_min,
    //     y_max-c.y_min,
    //     z_min-c.z_min,
    //     z_max-c.z_min

    // );
    // }

    // Maintain a list of cubes
    // Iterate over cuboids
    // When a cuboid intersects any cube, split that cube
    // Also, the list of 20 first do not interect the remaining 400, so
    // we can just add those
    //


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
