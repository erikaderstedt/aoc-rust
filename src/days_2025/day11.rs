// https://adventofcode.com/2025/day/11

use pathfinding::prelude::count_paths;
use std::str::FromStr;
use crate::common::{Solution, parsed_from_each_line};

pub fn solve(input: &str) -> Solution {
    let nodes: Vec<Node> = parsed_from_each_line(input);
    let you = Node::str_to_id("you");
    let out = Node::str_to_id("out");
    let svr = Node::str_to_id("svr");
    let dac = Node::str_to_id("dac");
    let fft = Node::str_to_id("fft");

    let successors = |&state: &usize| nodes.iter().find(|n| n.id == state).map_or(vec![], |n| n.outputs.clone());

    let p1 = count_paths(you, successors, |&state| state == out);

    let fft_to_dac = count_paths(fft, successors, |&state| state == dac);
    
    let p2 = if fft_to_dac > 0 {
        count_paths(svr, successors, |&state| state == fft) * 
        fft_to_dac *
        count_paths(dac, successors, |&state| state == out)
    } else {
        count_paths(svr, successors, |&state| state == dac) *
        count_paths(dac, successors, |&state| state == fft) *
        count_paths(fft, successors, |&state| state == out)
    };

    Solution::new(p1, p2)
}

struct Node {
    id: usize,
    outputs: Vec<usize>
}

impl Node {
    fn str_to_id(s: &str) -> usize {
        let l = &s.as_bytes()[0..3];
        ((l[0] as usize) << 16) + ((l[1] as usize) << 8) + (l[2] as usize)
    }
}

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Node::str_to_id(s);
        let outputs = s.split(' ').skip(1).map(|l| Node::str_to_id(l)).collect();
        Ok( Node { id, outputs })
    }
}
