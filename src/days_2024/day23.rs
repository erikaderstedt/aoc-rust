// https://adventofcode.com/2024/day/23

use std::collections::HashSet;
use itertools::Itertools;
use crate::common::Solution;

type Computer = u16;

fn computer_starts_with_t(c: &Computer) -> bool {
    (c >> 8) == (b't' as Computer)
}

fn computer_from(v: &[u8]) -> Computer {
    let b1 = (v[0] as Computer) << 8;
    let b2 = v[1] as Computer;
    b1 + b2
}
fn repr(c: &Computer) -> String {
    let v = vec![(c >> 8) as u8, (c & 255) as u8];
    std::str::from_utf8(&v).unwrap().to_string()
}

fn expand_set(set: HashSet<Computer>, connections: &Vec<(Computer,Computer)>) -> HashSet<Computer> {
    let mut nset = set
    .iter()
    .map(|c| -> HashSet<Computer> {
        connections
        .iter()
        .filter_map(|(c3,c4)| {
            // c3 in set, c4 not in set
            if c3 == c && !set.contains(c4) {
                Some(c4.clone())
            } else if c4 == c && !set.contains(c3) {
                Some(c3.clone())
            } else {
                None
            }
        })
        .collect()
    })
    .reduce(|acc, s| acc.intersection(&s).cloned().collect())
    .unwrap();

    for i in set.into_iter() {
        nset.insert(i);
    }

    // Remove from set if not connected to all others
    nset.iter()
    .filter(|&c| {
        // Get set of all connections to c
        let m: HashSet<Computer> = connections.iter()
        .filter_map(|(c1,c2)| {
            if c1 == c { 
                Some(c2) 
            } else if c2 == c { 
                Some(c1) 
            } else { 
                None 
            }
        })
        .cloned()
        .collect();

        let mut s2 = nset.clone();
        s2.remove(c);
        s2.is_subset(&m)
    })
    .cloned()
    .collect()
}

pub fn solve(input: &str) -> Solution {
    let connections: Vec<(Computer,Computer)> = input
        .lines()
        .map(|line| {
            let c1 = computer_from(&line.as_bytes()[0..2]);
            let c2 = computer_from(&line.as_bytes()[3..5]);
            (c1,c2)
        })
    .collect();

    let three_computer_sets: HashSet<(Computer,Computer,Computer)> = connections
    .iter()
    .map(|(c1, c2)| -> Vec<(Computer,Computer,Computer)> {
        let c1_connections: HashSet<Computer> = 
        connections
        .iter()
        .filter_map(|(c3,c4)| if c3 == c1 && c4 != c2 { Some(c4.clone()) } else if c4 == c1 && c3 != c2 { Some(c3.clone()) } else { None })
        .collect();
        let c2_connections: HashSet<Computer> = 
        connections
        .iter()
        .filter_map(|(c3,c4)| if c3 == c2 && c4 != c1 { Some(c4.clone()) } else if c4 == c2 && c3 != c1 { Some(c3.clone()) } else { None })
        .collect();

        c1_connections
        .intersection(&c2_connections)
        .map(|c3| {
            let mut v = vec![c1.clone(), c2.clone(), c3.clone()];
            v.sort();
            (v[0], v[1], v[2])
        })
        .collect()
    })
    .flatten()
    .collect();

    let p1 = three_computer_sets.iter()
    .filter(|(c1,c2,c3)| 
    computer_starts_with_t(c1) || computer_starts_with_t(c2) || computer_starts_with_t(c3))
    .count();

    let largest_group = connections
    .iter()
    .map(|s| -> HashSet<Computer>{
        let mut set0: HashSet<Computer> = HashSet::new();
        set0.insert(s.0);
        set0.insert(s.1);
        let x = expand_set(set0, &connections);
        x
    })
    .max_by_key(|s| s.len())
    .map(|h| {
        let mut v = h.into_iter().collect::<Vec<Computer>>();
        v.sort();
        v
    })
    .unwrap();
    
    let p2 = largest_group.iter().map(|c| repr(c)).join(",");
    
    Solution::new(p1, p2)
}
