// https://adventofcode.com/2022/day/15
use crate::common::{Solution, parsed_from_each_line};
use std::str::{FromStr, from_utf8};
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    range: usize,
}

type Position = (isize,isize);

#[derive(Debug,Hash,Eq,PartialEq,Clone)]
struct Beacon {
    x: isize,
    y: isize,
}

impl Sensor {

    fn covers(&self, x: isize, y: isize) -> bool {
        self.x.abs_diff(x) + self.y.abs_diff(y) <= self.range
    }

    fn iterate_over_outside_edge(&self) -> EdgeIterator {
        EdgeIterator { sensor_x: self.x, sensor_y: self.y, edge: Edge::TopLeft, done: false, x: self.x, y: self.y - (self.range as isize) - 1 }
    }
}

const P1_ROW: isize = 2000000;
const P2_Y_MAX: isize = 4000000;
const P2_X_MAX: isize = 4000000;

pub fn solve(input: &str) -> Solution {
    let sensors: Vec<Sensor> = parsed_from_each_line(input);
    let beacons: HashSet<Beacon> = parsed_from_each_line(input).into_iter().collect();

    let min_x = sensors.iter().map(|s| s.x - (s.range as isize)).min().unwrap();
    let max_x = sensors.iter().map(|s| s.x + (s.range as isize)).max().unwrap();

    let mut p1 = 0;
    for x in (min_x-1)..=(max_x+1) {
        let a = sensors.iter().any(|s| s.covers(x, P1_ROW));        
        if a {
            p1 += 1;
        }
    }
    p1 -= beacons.iter().filter(|b| b.y == P1_ROW).count();

    let p2 = sensors.iter().find_map(|sensor|
        match sensor
                .iterate_over_outside_edge()
                .filter(|(x,y)| *x >= 0 && *x <= P2_X_MAX && *y >= 0 && *y <= P2_Y_MAX)
                .find(|p| !sensors
                            .iter()
                            .any(|s| s.covers(p.0,p.1))
                ) {
            Some((x,y)) => {
                let p2 = x * 4000000 + y;
                Some(p2)
            },
            None => None,
        }).unwrap();

    Solution::new(p1,p2)
}

impl FromStr for Sensor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w: Vec<&str> = s.split(" ").collect();
        let x = from_utf8(&w[2].as_bytes()[2..(w[2].as_bytes().len()-1)]).unwrap().parse::<isize>().unwrap();
        let y = from_utf8(&w[3].as_bytes()[2..(w[3].as_bytes().len()-1)]).unwrap().parse::<isize>().unwrap();
        let closest_beacon_x = from_utf8(&w[8].as_bytes()[2..(w[8].as_bytes().len()-1)]).unwrap().parse::<isize>().unwrap();
        let closest_beacon_y = from_utf8(&w[9].as_bytes()[2..(w[9].as_bytes().len()-0)]).unwrap().parse::<isize>().unwrap();
        let range = x.abs_diff(closest_beacon_x) + y.abs_diff(closest_beacon_y);
        Ok(Sensor { x, y, range })
    }
}

impl FromStr for Beacon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w: Vec<&str> = s.split(" ").collect();
        let x = from_utf8(&w[8].as_bytes()[2..(w[8].as_bytes().len()-1)]).unwrap().parse::<isize>().unwrap();
        let y = from_utf8(&w[9].as_bytes()[2..(w[9].as_bytes().len()-0)]).unwrap().parse::<isize>().unwrap();
        Ok(Beacon { x,y })
    }
}

enum Edge {
    TopLeft,
    BottomLeft,
    BottomRight,
    TopRight,
}

struct EdgeIterator {
    sensor_x: isize,
    sensor_y: isize,
    edge: Edge,
    done: bool,
    x: isize,
    y: isize,

}

impl Iterator for EdgeIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None }

        let reached_corner = 
        match self.edge {
            Edge::TopLeft => { self.x -= 1; self.y += 1; self.y == self.sensor_y },
            Edge::BottomLeft => { self.x += 1; self.y += 1; self.x == self.sensor_x },
            Edge::BottomRight => { self.x += 1; self.y -= 1; self.y == self.sensor_y },
            Edge::TopRight => { self.x -= 1; self.y -= 1; self.x == self.sensor_x },
        };
        if reached_corner {
            self.edge = match self.edge {
                Edge::BottomLeft => Edge::BottomRight,
                Edge::BottomRight => Edge::TopRight,
                Edge::TopRight => { self.done = true; Edge::TopLeft },
                Edge::TopLeft => Edge::BottomLeft,
            };            
        }
        Some((self.x, self.y))
    }
}