use crate::common::Solution;
use std::collections::HashSet;

type Coordinate = (i8,i8,i8,i8);

fn perform_boot_cycle(initial_active_locations: &HashSet<Coordinate>, neighbors: &Vec<Coordinate>) -> usize {
    let mut active_locations = initial_active_locations.clone();
    for _ in 0..6 {
        let mut next_iteration: HashSet<Coordinate> = HashSet::new();

        // Add all potential sites to hashmap
        let mut possible_locations = active_locations.clone();
        for a in active_locations.iter() {
            for n in neighbors.iter() {
                possible_locations.insert((a.0+n.0,a.1+n.1,a.2+n.2,a.3+n.3));
            }
        }
        
        for a in possible_locations.iter() {
            // Check num active neighbors
            let is_active = active_locations.contains(a);
            match neighbors.iter().filter(|n| active_locations.contains(&(a.0+n.0,a.1+n.1,a.2+n.2,a.3+n.3))).count() {
                2 | 3 if is_active => { next_iteration.insert(a.clone()); },
                3 if !is_active => { next_iteration.insert(a.clone()); },
                _ => {},
            }
        }
        active_locations = next_iteration;
    }
    active_locations.len()
}

pub fn solve(input: &str) -> Solution {
    let initial_active_locations: HashSet<Coordinate> = input.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter(|(_,c)| *c == '#').map(move |(x,_)| (x as i8,y as i8,0i8,0i8))
        })
        .flatten()
        .collect();

    let p1_neighbors: Vec<Coordinate> = iproduct!(-1..=1, -1..=1, -1..=1)
                            .filter(|a| a.0 != 0 || a.1 != 0 || a.2 != 0)
                            .map(|a| (a.0,a.1,a.2,0))
                            .collect();
    let p2_neighbors: Vec<Coordinate> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
                            .filter(|a| a.0 != 0 || a.1 != 0 || a.2 != 0 || a.3 != 0).collect();

    let p1 = perform_boot_cycle(&initial_active_locations, &p1_neighbors);
    let p2 = perform_boot_cycle(&initial_active_locations, &p2_neighbors);

    Solution::new(p1,p2)
}
