use crate::common::Solution;
use regex::Regex;
use std::collections::HashMap;


pub fn solve(lines: &[String]) -> Solution {
    let re = Regex::new(r"^(\S+)\)(\S+)$").unwrap();
    let orbits: HashMap<String,String> = lines.iter().filter_map(|x| {
        match re.captures_iter(x).next() {
            Some(c1) => Some((c1[2].to_string(), c1[1].to_string())),
            None => None,
        }
    }).collect();

    let num_orbits = orbits.iter().fold(0u64,|acc, o| {
        let mut center = o.1;
        let mut x = acc + 1;
        while orbits.contains_key(center) {
            center = &orbits[center];
            x = x + 1
        }
        x
    });

    let mut ancestors_of_san: Vec<String> = Vec::new();
    let mut pos = "SAN".to_string();
    while orbits.contains_key(&pos) {
        pos = orbits[&pos].clone();
        ancestors_of_san.push(pos.clone());
    }
    let mut pos = "YOU".to_string();
    let mut distance = 0;
    while orbits.contains_key(&pos) && !ancestors_of_san.contains(&pos) {
        distance = distance + 1;
        pos = orbits[&pos].clone();
    }

    distance = distance + ancestors_of_san.into_iter().position(|p| p == pos).unwrap() + 1;
    // the above distance is the number of steps to go from YOU to SAN, but that isn't the question.
    distance = distance - 2; 

    Solution { part_1: num_orbits.to_string(), part_2: distance.to_string() }
}