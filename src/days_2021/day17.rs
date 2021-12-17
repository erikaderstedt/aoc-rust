// https://adventofcode.com/2021/day/17
use crate::common::Solution;

const TARGET_AREA_X_START: isize = 124;
const TARGET_AREA_X_STOP: isize = 174;
const TARGET_AREA_Y_START: isize = -123;
const TARGET_AREA_Y_STOP: isize = -86;

fn fire_probe(mut vx: isize, mut vy: isize) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    while y >= TARGET_AREA_Y_START {
        x += vx;
        y += vy;
        if y > max_y { max_y = y }

        match (x,y) {
            (TARGET_AREA_X_START..=TARGET_AREA_X_STOP, 
             TARGET_AREA_Y_START..=TARGET_AREA_Y_STOP) => { return Some(max_y) },
             _ => {
                if vx > 0 { vx -= 1; } else if vx < 0 { vx += 1; }
                vy -= 1;        
            },
        }
    }
    None
}

pub fn solve(_input: &str) -> Solution {
    let mut m1 = 0;
    let mut m2 = 0;
    for vy in TARGET_AREA_Y_START..150 {
        for vx in 10..=TARGET_AREA_X_STOP {
            if let Some(m) = fire_probe(vx, vy) {
                if m1 < m { m1 = m; }
                m2 += 1;
            }
        }
    }

    Solution::new(m1,m2)
}
