// https://adventofcode.com/2024/day/23

use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::bfs_reach;

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

fn find_connected(computer: &Computer, connections: &Vec<(Computer,Computer)>) -> Vec<Computer> {

    bfs_reach(computer.clone(), |c| {
        let destinations: Vec<Computer> = connections
            .iter()
            .filter(|(c1,c2)| c1 == c || c2 == c)
            .map(|(c1,c2)| if c1 == c { *c2 } else { *c1 })
            .collect();

        // println!("computer {} has these destinations: {:?}", repr(c), destinations.iter().map(|c| repr(c)).join(","));
            destinations
    })
    .collect()

    // let f: Vec<Computer> = found.iter().chain(vec![computer]).cloned().collect();
    // connections
    // .iter()
    // .filter(|(c1,c2)| (c1 == computer && !f.contains(c2)) || (c2 == computer && !f.contains(c1)))
    // .map(|(c1,c2)| -> Vec<Computer> {
    //     if c1 == computer && !f.contains(c2) {
    //         println!("Connection {} {}, found {:?} following {}", repr(c1), repr(c2), f.iter().map(|c| repr(c)).join(","), repr(c2));
    //         find_connected(c2, connections, &f)
    //     } else if c2 == computer && !f.contains(c1) {
    //         println!("Connection {} {}, found {:?} following {}", repr(c1), repr(c2), f.iter().map(|c| repr(c)).join(","), repr(c1));
    //         find_connected(c1, connections, &f)
    //     } else {
    //         panic!("?")
    //     }
    // })
    // .flatten()
    // .collect()
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

    let computers: HashSet<Computer> = connections
    .iter()
    .map(|(c1,c2)| vec![c1,c2])
    .flatten()
    .cloned()
    .collect();


    let three_computer_sets: HashSet<(Computer,Computer,Computer)> = computers
    .iter()
    .map(|c1| -> Vec<(Computer,Computer,Computer)> {
            // c1-c2 c2-c3 c3-c1
            let c2s: Vec<Computer> = connections
            .iter()
            .filter(|(a,b)| a == c1 || b == c1)
            .map(|(a,b)| if a == c1 { *b } else { *a })
            .collect();

            // println!("Computer {} is connected to {}", repr(c1), c2s.iter().map(|c| repr(c)).join(","));


            let sets: Vec<(Computer,Computer,Computer)> = c2s.iter().map(|c2| -> Vec<(Computer,Computer)> {
                connections
                .iter()
                .filter(|(a,b)| (a == c2 && b != c1) || (b == c2 && a != c1))
                .map(|(a,b)| {
                    if a == c2 {
                        (a.clone(), b.clone())
                    } else {
                        (b.clone(), a.clone())
                    }
                })
                .collect()
            })
            .flatten()
            .filter(|(_c2,c3)| {
                connections.iter().any(|(a,b)| (a == c3 && b == c1) || (b == c3 && a == c1))
            })
            .map(|(c2,c3)| {
                let mut v = vec![c2.clone(), c3.clone(), c1.clone()];
                v.sort();
                (v[0],v[1],v[2])
            })
            .collect();


        sets
    })
    .flatten()
    .collect();

    let p1 = three_computer_sets.iter()
    .filter(|(c1,c2,c3)| 
    computer_starts_with_t(c1) || computer_starts_with_t(c2) || computer_starts_with_t(c3))
    .count();
    
    let mut largest_group: Vec<Computer> = computers
    .iter()
    .map(|c| find_connected(c, &connections))
    .max_by_key(|c| c.len())
    .unwrap();
    largest_group.sort();

    let p2 = largest_group.iter().map(|c| repr(c)).join(",");
    
    Solution::new(p1, p2)
}
