// https://adventofcode.com/2021/day/22
use crate::common::Solution;
use regex::Regex;
use std::str::FromStr;
use crate::common::parsed_from_each_line;

#[derive(Debug)]
enum FlipState {
    On,
    Off,
}

#[derive(Debug)]
struct Cuboid {
    flip: FlipState,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Cuboid {
    fn touches(&self, x: isize, y: isize, z: isize) -> bool { 
        self.x_min <= x && self.x_max >= x &&
        self.y_min <= y && self.y_max >= y &&
        self.z_min <= z && self.z_max >= z
    }
}


pub fn solve(input: &str) -> Solution {
    let r = Regex::new(r"^(on|off) x=(-?[0-9]+)\.\.(-?[0-9]+),y=(-?[0-9]+)\.\.(-?[0-9]+),z=(-?[0-9]+)\.\.(-?[0-9]+)$").expect("Bad regex");

    let cuboids: Vec<Cuboid> = input.lines().map(|line| -> Cuboid {
        let cap = r.captures_iter(line).next().unwrap();
        Cuboid { flip: if &cap[1] == "on" { FlipState::On } else { FlipState::Off },
            x_min: cap[2].parse::<isize>().unwrap(),
            x_max: cap[3].parse::<isize>().unwrap(),
            y_min: cap[4].parse::<isize>().unwrap(),
            y_max: cap[5].parse::<isize>().unwrap(),
            z_min: cap[6].parse::<isize>().unwrap(),
            z_max: cap[7].parse::<isize>().unwrap(),
        }
    }).collect();

    let m2 = 0;
    const SIZE: usize = 102;
    const HSIZE: isize = 51;

    let m1 = {
        let mut grid = [[[0u8;SIZE];SIZE];SIZE];
        for c in cuboids.iter().take(20) {
            let mut j = 0;
            let v = match c.flip { FlipState::On => 1, FlipState::Off => 0 };
            for z in c.z_min..=c.z_max {
                for y in c.y_min..=c.y_max {
                    for x in c.x_min..=c.x_max {
                        j += 1;
                        grid[(z+HSIZE) as usize][(y+HSIZE) as usize][(x+HSIZE) as usize] = v;
                    }
                }
            }
        }
        let mut i: usize = 0;
        for z in 0..SIZE {
            for y in 0..SIZE {
                for x in 0..SIZE {
                    i += grid[z][y][x] as usize;
                }
            }
        }
        i
    };

    let x_min = cuboids.iter().map(|c| c.x_min).min().unwrap();
    let x_max = cuboids.iter().map(|c| c.x_max).max().unwrap();
    let y_min = cuboids.iter().map(|c| c.y_min).min().unwrap();
    let y_max = cuboids.iter().map(|c| c.y_max).max().unwrap();
    let z_min = cuboids.iter().map(|c| c.z_min).min().unwrap();
    let z_max = cuboids.iter().map(|c| c.z_max).max().unwrap();
    println!("{} {} {} {} {} {}", x_min, x_max, y_min, y_max, z_min, z_max);

    for c in cuboids.iter() {
        println!("{} -> {}, {} -> {}, {} -> {}", 
        x_min-c.x_min,
        x_max-c.x_min,
        y_min-c.y_min,
        y_max-c.y_min,
        z_min-c.z_min,
        z_max-c.z_min

    );
    }

    // Maintain a list of cubes
    // Iterate over cuboids
    // When a cuboid intersects any cube, split that cube
    // Also, the list of 20 first do not interect the remaining 400, so
    // we can just add those
    //


    Solution::new(m1,m2)
}

