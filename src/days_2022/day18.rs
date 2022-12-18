// https://adventofcode.com/2022/day/18

use crate::common::{Solution, parsed_from_each_line};
use itertools::Itertools;
use std::str::FromStr;

const SIDE: usize = 24;
const AIR: u8 = 0;
const OUTSIDE_AIR: u8 = 2;
const LAVA: u8 = 1;

type Space = [[[u8; SIDE];SIDE];SIDE];

#[derive(Debug)]
struct Droplet {
    x: usize,
    y: usize,
    z: usize,
}

impl Droplet {
    fn num_free_edges(&self, space: &Space) -> usize {
        let mut p = 0;
        if space[self.x-1][self.y][self.z] == AIR { p += 1; } 
        if space[self.x+1][self.y][self.z] == AIR { p += 1; } 
        if space[self.x][self.y-1][self.z] == AIR { p += 1; } 
        if space[self.x][self.y+1][self.z] == AIR { p += 1; } 
        if space[self.x][self.y][self.z-1] == AIR { p += 1; } 
        if space[self.x][self.y][self.z+1] == AIR { p += 1; } 
        p
    }
}

pub fn solve(input: &str) -> Solution {
    let droplets: Vec<Droplet> = parsed_from_each_line(input);
    
    let mut space = [[[AIR; SIDE];SIDE];SIDE];

    for d in droplets.iter() {
        space[d.x][d.y][d.z] = LAVA;
    }

    let p1 = droplets.iter().map(|d| d.num_free_edges(&space)).sum::<usize>();

    // Seed the sides with outside air.
    let mut exterior_spaces = 0usize;
    for i1 in 0..SIDE {        
        for i2 in 0..SIDE { 
            space[0][i1][i2] = OUTSIDE_AIR;
            space[i1][0][i2] = OUTSIDE_AIR;
            space[i1][i2][0] = OUTSIDE_AIR;
            space[SIDE-1][i1][i2] = OUTSIDE_AIR;
            space[i1][SIDE-1][i2] = OUTSIDE_AIR;
            space[i1][i2][SIDE-1] = OUTSIDE_AIR;
            exterior_spaces += 6;
        }
    }

    // Flood fill air with outside air.
    let mut exterior_spaces_at_iteration_start = 0;
    while exterior_spaces_at_iteration_start != exterior_spaces {
        exterior_spaces_at_iteration_start = exterior_spaces;
        for z in 1..(SIDE-1) {
            for y in 1..(SIDE-1) {
                for x in 1..(SIDE-1) {
                    if space[x][y][z] == AIR && (space[x-1][y][z] == OUTSIDE_AIR ||
                                                    space[x+1][y][z] == OUTSIDE_AIR ||
                                                    space[x][y-1][z] == OUTSIDE_AIR ||
                                                    space[x][y+1][z] == OUTSIDE_AIR ||
                                                    space[x][y][z-1] == OUTSIDE_AIR ||
                                                    space[x][y][z+1] == OUTSIDE_AIR) {
                        space[x][y][z] = OUTSIDE_AIR;
                        exterior_spaces += 1;
                    }
                }
            }
        }
    }

    let p2 = p1 - droplets.iter().map(|d| d.num_free_edges(&space)).sum::<usize>();

    Solution::new(p1,p2)
}

impl FromStr for Droplet {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match line.split(",").map(|i| i.parse::<usize>().unwrap()).collect_tuple() {
            Some((x,y,z)) => Ok(Droplet { x: x + 1, y: y + 1, z: z + 1 }),
            None => Err("Could not parse droplet")
        }
    }
}
