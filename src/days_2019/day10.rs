use crate::common::Solution;

pub fn solve(input: &str) -> Solution {
    // Convert grid into a set of asteroid positions.
    let mut asteroid_positions: Vec<(i64,i64)> = Vec::new();
    for (y,row_data) in input.lines().enumerate() {
        for (x, ch) in row_data.chars().enumerate() {
            if ch == '#' {
                asteroid_positions.push((x as i64,y as i64));
            }
        }
    }

    // Iterate over asteroid positions
    let number_of_visible_asteroids_at_best_position = asteroid_positions.iter().map(|pos| -> usize {
        asteroid_positions.iter().filter(|other_pos| -> bool { 
            let x_diff = other_pos.0 - pos.0;
            let y_diff = other_pos.1 - pos.1;

            // If x_diff is zero or y_diff is zero -- what?

            // If x_diff or y_diff are prime, then it is visible
            // Go up to sqrt(min(x_diff,y_diff))
            // Are there shared factors? 
            // 3*5, 3*4. Then check if 1*5,1*4 and 2*5,2*4 are occupied.
            let min_diff = if x_diff < y_diff { x_diff } else { y_diff };
            let sq_min_diff = (min_diff.abs() as f32).sqrt() as i64;
            for n in 1..sq_min_diff {
                if (x_diff % n) == 0 && (y_diff % n) == 0 {
                    // n is a shared factor.
                    for check in 1..n {
                        if asteroid_positions.contains(&(pos.0 + check*x_diff.signum(), pos.1 + check*y_diff.signum())) {
                            return false;
                        }
                    }
                }
            }
            true
        }).count()
        // Get a clone of all other asteroid positions.
        // If d
        // Go out one step. For positions that are in the asteroid list, calculate the diff to the original position.
        // Filter the other asteroid positions so that 


    }).max().unwrap();


    Solution::new(number_of_visible_asteroids_at_best_position, 
        "?")
}