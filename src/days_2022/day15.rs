// https://adventofcode.com/2022/day/15
use crate::common::{Solution, parsed_from_each_line};
use std::str::{FromStr, from_utf8};
use itertools::Itertools;
use std::ops::Range;

struct Sensor { x: isize, y: isize, beacon: (isize, isize), range: usize, }

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

    fn zero_intersects_negative_slope(&self) -> Vec<isize> {
        let r = self.range as isize;
        vec![
            self.x - (self.y + r) - 1, // bottom left edge
            self.x - (self.y - r) + 1, // top right edge
        ]
    }

    fn zero_intersects_positive_slope(&self) -> Vec<isize> {
        let r = self.range as isize;
        vec![
            self.x + (self.y + r) + 1, // bottom right edge
            self.x + (self.y - r) - 1, // top left edge
        ]
    }

    fn covers(&self, x: isize, y: isize) -> bool {
        self.x.abs_diff(x) + self.y.abs_diff(y) <= self.range
    }
} 

const P1_ROW: isize = 2000000;
const P2_Y_MAX: isize = 4000000;
const P2_X_MAX: isize = 4000000;

fn ranges_at_y(sensors: &Vec<Sensor>, y: isize) -> Vec<Range<isize>> {
    let mut ranges: Vec<Range<isize>> = sensors
        .iter()
        .filter_map(|sensor| sensor.range_at_row(y))
        .sorted_by_key(|range| range.start)
        .rev()
        .collect();

    match ranges.pop() {
        Some(mut current) => {
            let mut r = vec![];
            loop {
                if let Some(next) = ranges.pop() {
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
    let num_beacons_at_p1_row = sensors.iter()
        .filter_map(|sensor| 
            if sensor.beacon.1 == P1_ROW {
                Some(sensor.beacon.0)
            } else {
                None
            })
        .sorted()
        .dedup()
        .count();

    let p1 = ranges_at_y(&sensors, P1_ROW).into_iter().map(|range| range.len()).sum::<usize>() - num_beacons_at_p1_row;

    let x_axis_locations_positive: Vec<isize> = sensors
        .iter()
        .map(|sensor| sensor.zero_intersects_positive_slope().into_iter())
        .flatten()
        .sorted()
        .dedup()
        .collect();

    let x_axis_locations_negative: Vec<isize> = sensors
        .iter()
        .map(|sensor| sensor.zero_intersects_negative_slope().into_iter())
        .flatten()
        .sorted()
        .dedup()
        .collect();
    
    let p2 = iproduct!(x_axis_locations_negative.iter(), x_axis_locations_positive.iter())
        .find_map(|(x1, x2)| {
            let d = x1.abs_diff(*x2) as isize;
            if d % 2 != 0 || x2 < x1 {
                None
            } else {
                let x = d / 2 + x1.min(x2);
                let y = d / 2 * (x2-x1).signum();
                if y >= 0 && y <= P2_Y_MAX && x >= 0 && x <= P2_X_MAX && sensors.iter().all(|sensor| !sensor.covers(x, y)) {
                    Some(x * 4000000 + y)
                } else {
                    None
                }
            }
        })
        .expect("Unable to find intersection in pt2.");

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
        Ok(Sensor { x, y, beacon: (closest_beacon_x, closest_beacon_y), range })
    }
}

