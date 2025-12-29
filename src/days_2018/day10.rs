// https://adventofcode.com/2018/day/10

use crate::common::Solution;
use crate::grid::{read_characters_off_grid, Grid, Position};
use itertools::Itertools;

#[derive(Debug)]
struct Particle {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Particle {
    fn from(data: (i64, i64, i64, i64)) -> Particle {
        Particle {
            x: data.0,
            y: data.1,
            vx: data.2,
            vy: data.3,
        }
    }

    fn advance(&mut self, steps: i64) {
        self.x = self.x + self.vx * steps;
        self.y = self.y + self.vy * steps;
    }

    fn steps_to_zero(&self) -> i64 {
        (self.x / self.vx).abs().min((self.y / self.vy).abs())
    }

    fn dimensions(particles: &Vec<Particle>) -> (i64, i64, i64, i64, i64) {
        let min_x = particles.iter().map(|p| p.x).min().unwrap();
        let min_y = particles.iter().map(|p| p.y).min().unwrap();
        let max_x = particles.iter().map(|p| p.x).max().unwrap();
        let max_y = particles.iter().map(|p| p.y).max().unwrap();
        let area = (max_x - min_x + 1) * (max_y - min_y + 1);
        (min_x, min_y, max_x, max_y, area)
    }
}

pub fn solve(input: &str) -> Solution {
    let mut particles: Vec<Particle> = input
        .lines()
        .map(|line| {
            Particle::from(
                line.replace("position=<", "")
                    .replace("> velocity=<", ",")
                    .replace(">", "")
                    .split(',')
                    .map(|s| s.trim().parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect();

    let skip = particles.iter().map(|p| p.steps_to_zero()).min().unwrap() - 10;
    for particle in particles.iter_mut() {
        particle.advance(skip);
    }
    let mut step = 0;
    let mut smallest_seen_area = i64::MAX;
    let p1 = loop {
        step = step + 1;
        for particle in particles.iter_mut() {
            particle.advance(1);
        }
        let area = Particle::dimensions(&particles).4;

        if area > smallest_seen_area {
            for particle in particles.iter_mut() {
                particle.advance(-1);
            }
            let (min_x, min_y, max_x, max_y, area) = Particle::dimensions(&particles);
            let mut grid = Grid {
                rows: (max_y - min_y + 1) as usize,
                cols: (max_x - min_x + 1) as usize,
                locations: vec![false; area as usize],
            };
            for particle in particles.iter() {
                let p = Position {
                    column: (particle.x - min_x) as usize,
                    row: (particle.y - min_y) as usize,
                };
                grid.set(&p, true);
            }

            break read_characters_off_grid::<6, 10, 2>(&grid).unwrap();
        } else {
            smallest_seen_area = area;
        }
    };

    let p2 = step + skip - 1;

    Solution::new(p1, p2)
}
