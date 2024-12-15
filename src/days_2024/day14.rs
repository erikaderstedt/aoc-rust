// https://adventofcode.com/2024/day/14

use std::str::FromStr;
use crate::common::{parsed_from_each_line, Solution};

const GRID_SIZE_X: isize = 101;
const GRID_SIZE_Y: isize = 103;

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn moved(self: &Self, step: isize) -> Robot {
        let px = (self.position.0 + self.velocity.0 * step).rem_euclid(GRID_SIZE_X);
        let py = (self.position.1 + self.velocity.1 * step).rem_euclid(GRID_SIZE_Y);
        
        Robot { position: (px, py), velocity: self.velocity }
    }

    fn distance_from_center(self: &Self) -> isize {
        (self.position.0 - GRID_SIZE_X / 2).pow(2) + (self.position.1 - GRID_SIZE_Y/2).pow(2)
    }
}

pub fn solve(input: &str) -> Solution {
    let robots: Vec<Robot> = parsed_from_each_line(input);
    let robots_after_100s: Vec<Robot> = robots.iter()
        .map(|robot: &Robot| -> Robot { robot.moved(100) })
        .collect();

    let q1 = robots_after_100s.iter().filter(|robot| robot.position.0 < 50 && robot.position.1 < 51).count();
    let q2 = robots_after_100s.iter().filter(|robot| robot.position.0 > 50 && robot.position.1 < 51).count();
    let q3 = robots_after_100s.iter().filter(|robot| robot.position.0 < 50 && robot.position.1 > 51).count();
    let q4 = robots_after_100s.iter().filter(|robot| robot.position.0 > 50 && robot.position.1 > 51).count();
    let p1 = q1 * q2 * q3 * q4;

    // Total of 101*103 possible configurations
    // Check where the robots are closest to the center.
    let p2 = (0..GRID_SIZE_X*GRID_SIZE_Y).map(|step| -> (isize, isize) {
        let total_distance_from_center = robots.iter().map(|robot| robot.moved(step as isize).distance_from_center()).sum();
        (step, total_distance_from_center)
    }).min_by_key(|(_, d)| *d).unwrap().0;

    Solution::new(p1, p2)
}

impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ps, vs) = s.split_once(' ').unwrap();
        let (px, py) = ps[2..].split_once(',').unwrap();
        let (vx, vy) = vs[2..].split_once(',').unwrap();
        let pxv = px.parse::<isize>().unwrap();
        let pyv = py.parse::<isize>().unwrap();
        let vxv = vx.parse::<isize>().unwrap();
        let vyv = vy.parse::<isize>().unwrap();

        Ok( Robot { position: (pxv, pyv), velocity: (vxv, vyv) })
    }
}
