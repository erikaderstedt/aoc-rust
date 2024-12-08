// https://adventofcode.com/2023/day/25

use crate::common::Solution;
use itertools::Itertools;
use pathfinding::prelude::bfs;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{HashMap, HashSet};

type Graph = HashMap<u32, Vec<u32>>;

fn encode(s: &str) -> u32 {
    let q = s.as_bytes();
    ((q[0] as u32) << 16) + ((q[1] as u32) << 8) + (q[2] as u32)
}

fn decode(u: u32) -> String {
    format!(
        "{}{}{}",
        ((u >> 16) as u8) as char,
        (((u >> 8) & 0xFF) as u8) as char,
        ((u & 0xFF) as u8) as char
    )
}

pub fn solve(input: &str) -> Solution {
    let all_nodes: Vec<u32> = input
        .lines()
        .map(|line| -> Vec<&str> { line.split(' ').collect() })
        .flatten()
        .map(|s| encode(s))
        .collect::<HashSet<u32>>()
        .into_iter()
        .collect();

    let mut bidirectional_graph: Graph = all_nodes
        .iter()
        .map(|node| -> (u32, Vec<u32>) {
            let node_s = decode(*node);
            (
                *node,
                input
                    .lines()
                    .filter(|line| line.contains(&node_s))
                    .map(|line| -> Vec<u32> {
                        if line.starts_with(&node_s) {
                            line.split(' ').skip(1).map(|s| encode(s)).collect()
                        } else {
                            vec![encode(line)]
                        }
                    })
                    .flatten()
                    .collect(),
            )
        })
        .collect();

    let mut traversed_edges: HashMap<u64, usize> = HashMap::new();
    let mut rng = thread_rng();
    for _ in 0..1000 {
        // Grab two random keys in the bidirectional graph
        let node1: u32 = *all_nodes.choose(&mut rng).unwrap();
        let node2: u32 = *all_nodes.choose(&mut rng).unwrap();
        let p = bfs(
            &node1,
            |state: &u32| bidirectional_graph[state].iter().cloned(),
            |state| *state == node2,
        )
        .unwrap();

        // For each edge traversed, increase or set edge in hashmap
        for node_pair in p.windows(2) {
            let edge = ((std::cmp::min(node_pair[0], node_pair[1]) as u64) << 32)
                + (std::cmp::max(node_pair[0], node_pair[1]) as u64);
            match traversed_edges.get_mut(&edge) {
                Some(cnt) => *cnt += 1,
                None => {
                    traversed_edges.insert(edge, 1);
                }
            }
        }
    }

    // Remove the three most traversed edges.
    for (n1, n2) in traversed_edges
        .into_iter()
        .sorted_unstable_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(3)
        // .inspect(|(h, c)| {
        //     println!(
        //         "{} hits on {:?} to {:?}",
        //         c,
        //         decode((h >> 32) as u32),
        //         decode((h & 0xFFFF_FFFF) as u32)
        //     )
        // })
        .map(|(h, _)| ((h >> 32) as u32, (h & 0xFFFF_FFFF) as u32))
    {
        bidirectional_graph.get_mut(&n1).unwrap().retain(|d| *d != n2);
        bidirectional_graph.get_mut(&n2).unwrap().retain(|d| *d != n1);
    }

    // Select a random node
    // Enumerate nodes
    // Among the nodes, how many can reach one of the nodes on the deleted edge?
    // This number * (total number of nodes - this number) => answer
    let target: u32 = *all_nodes.choose(&mut rng).unwrap();
    let num_nodes = all_nodes.len();
    let can_reach_target = all_nodes
        .into_iter()
        .filter(|n| -> bool {
            bfs(
                n,
                |state: &u32| bidirectional_graph[state].iter().cloned(),
                |state| *state == target,
            )
            .is_some()
        })
        // .inspect(|n| println!("{:?} can reach {:?}", decode(*n), decode(target)))
        .count();

    let p1 = can_reach_target * (num_nodes - can_reach_target);
    let p2 = 0;

    Solution::new(p1, p2)
}
