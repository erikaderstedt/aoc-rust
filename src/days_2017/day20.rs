// https://adventofcode.com/2017/day/20

use crate::common::Solution;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl Vector {
    fn squared_norm(&self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Particle {
    fn propagate(&mut self) {
        self.v.x += self.a.x;
        self.v.y += self.a.y;
        self.v.z += self.a.z;
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        self.p.z += self.v.z;
    }
}

impl FromStr for Vector {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s[3..(s.len() - 1)]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y, z })
    }
}

pub fn solve(input: &str) -> Solution {
    let mut particles: Vec<Particle> = input
        .lines()
        .map(|line| {
            let (p, v, a) = line
                .split(", ")
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Particle { p, v, a }
        })
        .collect();

    let p1 = particles
        .iter()
        .enumerate()
        .min_by_key(|(_, particle)| particle.a.squared_norm())
        .unwrap()
        .0;

    // No fancy logic to detect when no more collisions are possible.
    // For my input the last collision is at around iteration 40.
    for _ in 0..50 {
        for particle in particles.iter_mut() {
            particle.propagate();
        }
        let mut i = 0;
        while i < particles.len() {
            let mut collided = false;
            while let Some(j) = particles
                .iter()
                .skip(i + 1)
                .position(|other| particles[i].p == other.p)
            {
                particles.remove(j + i + 1);
                collided = true;
            }
            if collided {
                particles.remove(i);
            } else {
                i += 1;
            }
        }
    }
    let p2 = particles.len();

    Solution::new(p1, p2)
}
