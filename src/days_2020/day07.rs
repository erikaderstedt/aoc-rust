use crate::common::Solution;
use std::collections::{HashMap,HashSet};

fn num_contained_bags(data: &HashMap<&str,Vec<(usize,&str)>>, bag: &str) -> usize {
    data[bag].iter().map(|(num, c)| num * (num_contained_bags(data, &c) + 1)).sum()
}

pub fn solve(input: &str) -> Solution {
    
    let data: HashMap<&str,Vec<(usize,&str)>> = input.lines().map(|line| {
        let i = line.find(" bags contain").unwrap();
        let containees = &line[(i + " bags contain".len())..];
        (&line[0..i], containees
            .split(|c| c == ',' || c == '.')
            .filter(|s| s.len() > 0)
            .filter_map(|s| {
                if let Ok(num) = s[1..2].parse::<usize>() {
                    Some((num, s[3..(s.len()-4)].trim()))
                } else {
                    None
                }
            }).collect()
        )
    }).collect();

    let my_bag = "shiny gold";

    let mut can_hold_my_bag: HashSet<&str> = HashSet::new(); 
    loop {
        let i: Vec<&str> = data
        .iter()
        .filter_map(|(k, v)|             
            if !can_hold_my_bag.contains(k) && v.iter().any(|(_num, c)| can_hold_my_bag.contains(c) || *c == my_bag) {
                Some(*k)
            } else { None })
        .collect();
        if i.len() == 0 { break }
        can_hold_my_bag.extend(i.into_iter());
    }
 
    let p1 = can_hold_my_bag.len();
    let p2 = num_contained_bags(&data, &my_bag);

    Solution::new(p1,p2)
}
