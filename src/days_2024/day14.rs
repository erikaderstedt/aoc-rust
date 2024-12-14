// https://adventofcode.com/2024/day/14

use std::str::FromStr;
use crate::common::{parsed_from_each_line, Solution};

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn moved(self: &Self, grid_size: (isize, isize)) -> Robot {
        let px = (self.position.0 + self.velocity.0).rem_euclid(grid_size.0);
        let py = (self.position.1 + self.velocity.1).rem_euclid(grid_size.1);
        
        Robot { position: (px, py), velocity: self.velocity }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut robots: Vec<Robot> = parsed_from_each_line(input);

    let mut seconds = 0;
    let mut p1 = 0;
    let p2;
    loop {
        robots = robots.into_iter().map(|robot| robot.moved((101,103))).collect();
        seconds = seconds + 1;
    
        if seconds == 100 {
            let q1 = robots.iter().filter(|robot| robot.position.0 < 50 && robot.position.1 < 51).count();
            let q2 = robots.iter().filter(|robot| robot.position.0 > 50 && robot.position.1 < 51).count();
            let q3 = robots.iter().filter(|robot| robot.position.0 < 50 && robot.position.1 > 51).count();
            let q4 = robots.iter().filter(|robot| robot.position.0 > 50 && robot.position.1 > 51).count();
        
            p1 = q1 * q2 * q3 * q4;
        }

        // The edges have less than half the expected number of robots.
        let q5 = robots.iter().filter(|robot| robot.position.0 > 80).count();
        let q6 = robots.iter().filter(|robot| robot.position.0 < 20).count();
        let q7 = robots.iter().filter(|robot| robot.position.1 > 80).count();
        let q8 = robots.iter().filter(|robot| robot.position.1 < 20).count();
        if q5 < 50 && q6 < 50 && q7 < 50 && q8 < 50 {
            p2 = seconds;
            break;
        }
        
    }

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
