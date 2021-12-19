// https://adventofcode.com/2021/day/19
use crate::common::Solution;
use itertools::Itertools;
use std::str::FromStr;
use std::collections::{HashMap,HashSet};

type R = i16;

#[derive(Debug,PartialEq,Clone,Hash,Eq)]
struct Position {
    x: R,
    y: R,
    z: R,
}

#[derive(Debug,PartialEq,Clone,Hash,Eq)]
struct Delta {
    dx: R,
    dy: R,
    dz: R,
}


#[derive(Debug,PartialEq,Clone,Eq)]
struct Scanner {
    id: u32,
    pings: Vec<Position>,
    position: Option<Position>,
    hashes: HashSet<R>,
}

const MIN_OVERLAP: usize = 12;

impl Scanner {

    fn fix_orientation_and_apply_delta(&mut self, o: usize, delta: Delta) {
        self.pings = self.pings.iter().map(|p| p.rotated(o).translated(&delta)).collect();
        self.position = Some(Position { x: delta.dx, y: delta.dy, z: delta.dz });
    }

    fn check_overlap(&self, other_pings: Vec<Position>) -> Option<Delta> {
        let mut highest: usize = 0;
        let mut counts: HashMap<Delta, usize> = HashMap::new();
        for (i,ping) in self.pings.iter().enumerate() {
            for other_ping in other_pings.iter() {
                let delta = ping.delta(other_ping);
                counts.entry(delta.clone())
                    .and_modify(|e| {
                        *e += 1;
                        if *e > highest { highest = *e; }
                    })
                    .or_insert(1);
            }
            if highest >= MIN_OVERLAP { break; }
            if self.pings.len() - i < MIN_OVERLAP - highest {
                // Not even if all remaining beacons are a match will we reach 12.
                return None
            }
        }
        counts.into_iter().find_map(|(delta, c)| if c >= MIN_OVERLAP { Some(delta) } else { None })
    }
}

struct Wavefront {
    id: u32,            // Of newly matched scanner
    orientation: usize, // Of newly matched scanner
    delta: Delta,       // From discoverer to newly matched scanner
}

impl Wavefront {
    fn source() -> Wavefront { Wavefront { id: 0, orientation: 0, delta: Delta::zero() }}
}

pub fn solve(input: &str) -> Solution {
    let mut scanners: Vec<Scanner> = input.split("\n\n").map(|x| 
        match x.parse::<Scanner>() {
            Ok(v) => v,
            _ => panic!("Bad input"),
        }).collect();

    let mut oriented_scanners: Vec<Scanner> = Vec::new();

    let mut wavefront: Vec<Wavefront> = vec![Wavefront::source()];
    while wavefront.len() > 0 {
        // Accumulate delta from first scanner in oriented scanners.
        // Then we should be able to add all delta-adjusted pings into a vec, sort and then dedup and count.
        // Remove wavefront members from scanners
        let num_previously_oriented_scanners = oriented_scanners.len();
        while let Some(Wavefront { id, orientation, delta }) = wavefront.pop() {
            match scanners.iter().position(|s| s.id == id) {
                Some(index) => { 
                    let mut found = scanners.remove(index);
                    found.fix_orientation_and_apply_delta(orientation,delta);
                    oriented_scanners.push(found) }
                None => {  }
            }
        }

        let mut next_wavefront = Vec::new();
        for scanner in oriented_scanners.iter().skip(num_previously_oriented_scanners) {
            for remaining_scanner in scanners.iter() {
                if scanner.hashes.intersection(&remaining_scanner.hashes).count() < 66 /* (12 2) = 12!/10!/2 */ { continue; }

                for orientation in 0..NUM_ROTATIONS {
                    if let Some(delta) = scanner.check_overlap(
                        remaining_scanner.pings.iter().map(|p| p.rotated(orientation)).collect()) {
                        next_wavefront.push(
                            Wavefront { id: remaining_scanner.id, orientation, 
                                delta });
                        break;
                    }
                }
            }
        }
        wavefront = next_wavefront;
    }

    let m1 = {
        let h: HashSet<&Position> = oriented_scanners.iter()
            .map(|o| &o.pings)
            .flatten()
            .collect();
        h.len()
    };

    let m2 = oriented_scanners.iter()
        .filter_map(|s| s.position.clone())
        .permutations(2)
        .map(|a| a[0].manhattan_distance(&a[1]) )
        .max()
        .unwrap();

    Solution::new(m1,m2)
}


const NUM_ROTATIONS: usize = 24;

impl Position {

    fn delta(&self, other: &Position) -> Delta {
        Delta { dx: self.x - other.x, dy: self.y - other.y, dz: self.z - other.z }
    }

    fn manhattan_distance(&self, other: &Position) -> R {
        R::abs(self.x - other.x) +
        R::abs(self.y - other.y) +
        R::abs(self.z - other.z) 
    }

    fn translated(&self, delta: &Delta) -> Position {
        Position { x: self.x + delta.dx, y: self.y + delta.dy, z: self.z + delta.dz }
    }

    fn rotated(&self, n: usize) -> Position {
        match n {
            23 => Position { x:   self.x,  y:  self.y, z:  self.z },
            1  => Position { x:   self.x,  y:  self.z, z: -self.y },
            2  => Position { x:   self.x,  y: -self.y, z: -self.z },
            3  => Position { x:   self.x,  y: -self.z, z:  self.y },
            4  => Position { x:   self.y,  y:  self.x, z: -self.z },
            5  => Position { x:   self.y,  y:  self.z, z:  self.x },
            6  => Position { x:   self.y,  y: -self.x, z:  self.z },
            7  => Position { x:   self.y,  y: -self.z, z: -self.x },
            8  => Position { x:   self.z,  y:  self.x, z:  self.y },
            9  => Position { x:   self.z,  y:  self.y, z: -self.x },
            10 => Position { x:   self.z,  y: -self.x, z: -self.y },
            11 => Position { x:   self.z,  y: -self.y, z:  self.x },
            12 => Position { x:  -self.x,  y:  self.y, z: -self.z },
            13 => Position { x:  -self.x,  y:  self.z, z:  self.y },
            14 => Position { x:  -self.x,  y: -self.y, z:  self.z },
            15 => Position { x:  -self.x,  y: -self.z, z: -self.y },
            16 => Position { x:  -self.y,  y:  self.x, z:  self.z },
            17 => Position { x:  -self.y,  y:  self.z, z: -self.x },
            18 => Position { x:  -self.y,  y: -self.x, z: -self.z },
            19 => Position { x:  -self.y,  y: -self.z, z:  self.x },
            20 => Position { x:  -self.z,  y:  self.x, z: -self.y },
            21 => Position { x:  -self.z,  y:  self.y, z:  self.x },
            22 => Position { x:  -self.z,  y: -self.x, z:  self.y },
            0 => Position { x:   -self.z,  y: -self.y, z: -self.x },
            _ => unreachable!("Unknown orientation number.")
        }
    }

}

impl Delta {
    fn zero() -> Delta { Delta { dx: 0, dy: 0, dz: 0 } }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x,y,z)) = s.split(',').map(|v| v.parse::<R>().unwrap()).collect_tuple() {
            Ok(Position { x, y, z })
        } else {
            Err("Malformed ping self.")
        }
    }
}

impl FromStr for Scanner {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pings: Vec<Position> = s.lines().skip(1).map(|x| 
            match x.parse::<Position>() {
                Ok(v) => v,
                _ => panic!("Bad input"),
            }
        ).collect();
        let hashes: HashSet<R> = pings.iter()
            .combinations(2)
            .map(|a| a[0].manhattan_distance(&a[1]))
            .collect();
        if let Ok(id) = scan_fmt!( s, "--- scanner {d} ---", u32) {
            Ok(Scanner { id, pings: pings, hashes, position: None })
        } else {
            Err("Unable to parse scanner id.")
        }
    }
}