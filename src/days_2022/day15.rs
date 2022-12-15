// https://adventofcode.com/2022/day/15
use crate::common::{Solution, parsed_from_each_line};
use std::str::{FromStr, from_utf8};
use itertools::Itertools;
use std::ops::Range;
use std::collections::VecDeque;
use std::time::Instant;
struct Beacon { x: isize, y: isize }
struct Sensor { x: isize, y: isize, range: usize, }

impl Sensor {

    fn range_at_row(&self, row: isize) -> Option<Range<isize>> {
        let reach = (self.range - row.abs_diff(self.y)) as isize;
        if reach >= 0 {
            let length = 2 * (reach + 1) - 1;
            let start = self.x-reach;
            let end = start + length;
            Some(start..end)
        } else {
            None
        }
    }

    fn covers(&self, x: isize, y: isize) -> bool {
        self.x.abs_diff(x) + self.y.abs_diff(y) <= self.range
    }

    fn iterate_over_outside_edge(&self) -> EdgeIterator {
        EdgeIterator { sensor_x: self.x, sensor_y: self.y, edge: Edge::TopLeft, done: false, x: self.x, y: self.y - (self.range as isize) - 1 }
    }
} 

const P1_ROW: isize = 2000000;
// const P1_ROW: isize = 10;

const P2_Y_MAX: isize = 4000000;
const P2_X_MAX: isize = 4000000;

fn ranges_at_y(sensors: &Vec<Sensor>, y: isize) -> Vec<Range<isize>> {
    let mut ranges: VecDeque<Range<isize>> = sensors
        .iter()
        .filter_map(|sensor| sensor.range_at_row(y))
        .sorted_by_key(|range| range.start)
        .collect();

    match ranges.pop_front() {
        Some(mut current) => {
            let mut r = vec![];
            loop {
                if let Some(next) = ranges.pop_front() {
                    if next.start <= current.end {
                        current.end = next.end.max(current.end);
                    } else {
                        r.push(current);
                        current = next;
                    }
                } else {
                    r.push(current);
                    break r
                }
            }
        },
        None => vec![],
    }

}

pub fn solve(input: &str) -> Solution {
    let sensors: Vec<Sensor> = parsed_from_each_line(input);
    let num_beacons_at_p1_row = parsed_from_each_line::<Beacon>(input)
        .into_iter()
        .filter(|beacon| beacon.y == P1_ROW)
        .map(|beacon| beacon.x)
        .sorted()
        .dedup()
        .count();

    let mut p1 = 0;

    let start = Instant::now();        

    let p1 = ranges_at_y(&sensors, P1_ROW).into_iter().map(|range| range.len()).sum::<usize>() - num_beacons_at_p1_row;

    let duration = start.elapsed();

    println!("P1 Elapsed time: {:>7} µs", duration.as_micros());

    // let p2 = (0..P2_Y_MAX).find(|&y| ranges_at_y(&sensors, y).len() > 1).unwrap();

    let p2 = sensors.iter().find_map(|sensor|
        match sensor
                .iterate_over_outside_edge()
                .filter(|(x,y)| *x >= 0 && *x <= P2_X_MAX && *y >= 0 && *y <= P2_Y_MAX)
                .find(|(x, y)| !sensors
                            .iter()
                            .any(|s| s.covers(*x,*y))
                ) {
            Some((x,y)) => { println!("{} {}", x, y); Some(x * 4000000 + y) },
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
    type Item = (isize,isize);

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