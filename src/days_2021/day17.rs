// https://adventofcode.com/2021/day/17
use crate::common::Solution;
use itertools::Itertools;

type Position = i16;
type Velocity = i16;
type Iteration = u16;
const TARGET_AREA_X_START: Position = 124;
const TARGET_AREA_X_STOP: Position = 174;
const TARGET_AREA_Y_START: Position = -123;
const TARGET_AREA_Y_STOP: Position = -86;

// Get the lowest and highest iteration number that will hit the target area given this input velocity.
fn iterations<F: Fn(Position) -> bool, G: Fn(Position) -> bool>(mut velocity: Velocity, reached: F, passed: G, stop: bool) 
    -> Option<((Iteration, Iteration), Velocity)> {
    let mut p = 0;
    let mut n = 0;
    let v0 = velocity;
    let minimum_num_iterations = loop { 
        p += velocity; 
        velocity -= 1; 
        n += 1;
        if reached(p) { break n }
    };
    if passed(p) { None } // We blew right past. 
    else if stop { Some(((minimum_num_iterations, Iteration::MAX), v0)) }
    else {
        let maximum_num_iterations = loop { 
            p += velocity; 
            velocity -= 1; 
            if passed(p) { break n }
            n += 1;
        };
        Some(((minimum_num_iterations, maximum_num_iterations), v0))
    }
}

pub fn solve(_input: &str) -> Solution {
    // vx is at most TARGET_AREA_X_STOP
    // the cumulative sum of vx must also be at least TARGET_AREA_X_START
    // otherwise we will never reach the target area.
    let lowest_x_velocity: Velocity = (1..TARGET_AREA_X_START).find(|x| x*(x+1) >= 2*TARGET_AREA_X_START).unwrap();
    let x_data: Vec<((Iteration, Iteration), Velocity)> = (lowest_x_velocity..=TARGET_AREA_X_STOP)
        .flat_map(move |vx| iterations(vx, |x| x >= TARGET_AREA_X_START, |x| x > TARGET_AREA_X_STOP, vx*(vx+1)/2 <= TARGET_AREA_X_STOP))
        .collect();
    
    let y_data: Vec<((Iteration, Iteration), Velocity)> = (TARGET_AREA_Y_START..=(-TARGET_AREA_Y_START))
        .flat_map(move |vy| iterations(vy, |y| y <= TARGET_AREA_Y_STOP, |y| y < TARGET_AREA_Y_START, false))
        .collect();

    let highest_upward_velocity = y_data.iter().map(|&a| a.1).max().unwrap();
    let m1 = highest_upward_velocity*(highest_upward_velocity+1)/2;
    // Look at all combinations of x and y velocities that have an overlapping number of iterations
    // This means that for this combination of vx and vy there will be an iteration that overlaps.
    let m2 = x_data.into_iter()
        .cartesian_product(y_data.into_iter())
        .filter(|((x,_), (y,_))| !(x.1 < y.0 || y.1 < x.0))
        .count();

    Solution::new(m1,m2)
}
