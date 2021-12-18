// https://adventofcode.com/2021/day/17
use crate::common::Solution;

type Position = i32;
const TARGET_AREA_X_START: Position = 124;
const TARGET_AREA_X_STOP: Position = 174;
const TARGET_AREA_Y_START: Position = -123;
const TARGET_AREA_Y_STOP: Position = -86;

fn does_probe_hit(mut x: Position, mut y: Position, mut vx: Position, mut vy: Position) -> bool {
    while y >= TARGET_AREA_Y_START && x <= TARGET_AREA_X_STOP {
        if x >= TARGET_AREA_X_START && y <= TARGET_AREA_Y_STOP {
            return true;
        }
        x += vx;
        y += vy;
        if vx > 0 { vx -= 1; }
        vy -= 1;        
    }
    false
}

pub fn solve(_input: &str) -> Solution {
    let mut m1 = 0;
    let mut m2 = 0;
    // vx is at most TARGET_AREA_X_STOP
    // the cumulative sum of vx must also be at least TARGET_AREA_X_START
    // otherwise we will never reach the target area.
    let lowest_x_velocity: Position = (1..TARGET_AREA_X_START).find(|x| x*(x+1) >= 2*TARGET_AREA_X_START).unwrap();
    for vx in lowest_x_velocity..=TARGET_AREA_X_STOP {
        // vx * N >= TARGET_AREA_X_START
        // vx * N <= TARGET_AREA_X_STOP
        let mut x = 0;
        let mut n = 0;
        let mut vx_simulated = vx;
        let minimum_num_iterations = loop { 
            x += vx_simulated; 
            if vx_simulated > 0 { vx_simulated -= 1; }
            n += 1;
            if x >= TARGET_AREA_X_START { break n }
        };
        let minimum_vy_diff = minimum_num_iterations*(minimum_num_iterations-1)/2;
        let vy_lower_bound = (TARGET_AREA_Y_START + minimum_vy_diff)/minimum_num_iterations;

        let vy_upper_bound: Position = 
        if vx*(vx+1)/2 <= TARGET_AREA_X_STOP {
            // if vx*(vx+1)/2 <= TARGET_AREA_X_STOP, then vx will reach zero.there is no upper bound on max number of iterations
            // (because vx will be zero).
            // To determine the upper bound, realize that all trajectories with 
            // vy_initial > 0 will have a point I where y = 0 and vy = -vy_initial
            // If vy_initial is higher than TARGET_AREA_Y_START then we will miss with the next iteration.
            -TARGET_AREA_Y_START
        } else {
            let mut x1 = x;
            let mut vx_simulated1 = vx_simulated;
            let maximum_num_iterations = loop { 
                x1 += vx_simulated1; 
                if vx_simulated1 > 0 { vx_simulated1 -= 1; }
                n += 1;
                if x1 > TARGET_AREA_X_STOP { break n }
            };
            (TARGET_AREA_Y_STOP + maximum_num_iterations*(maximum_num_iterations-1)/2)/maximum_num_iterations
        };
        
        for vy in vy_lower_bound ..= vy_upper_bound {
            // We know that the simulation will take at least minimum_num_iterations.
            let vy_simulated = vy - minimum_num_iterations;
            let y = minimum_num_iterations*vy - minimum_vy_diff;
            if does_probe_hit(x, y, vx_simulated, vy_simulated) {
                let max_y = vy*(vy+1)/2;
                if m1 < max_y { m1 = max_y; }
                m2 += 1;
            }
        }
    }

    Solution::new(m1,m2)
}
