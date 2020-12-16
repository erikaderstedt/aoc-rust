use crate::common::Solution;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq,Eq)]
struct TicketField {
    name: String,
    range1: (usize, usize),
    range2: (usize, usize),
}

impl TicketField {
    fn valid_for(&self, v: usize) -> bool {
        (self.range1.0 <= v && v <= self.range1.1) ||
        (self.range2.0 <= v && v <= self.range2.1)
    }
}

fn ticket_to_vec(s: &str) -> Vec<usize> {
    s.split(',').map(|l| l.parse::<usize>().expect("")).collect()
}

pub fn solve(input: &str) -> Solution {
    let r = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").expect("Bad regex");

    let ranges: Vec<TicketField> = input.lines().take_while(|&l| l != "").map(|l| -> TicketField {
        let cap = r.captures_iter(l).next().unwrap();
        TicketField { name: cap[1].to_string(), 
            range1: (cap[2].parse::<usize>().expect(""),cap[3].parse::<usize>().expect("")),
            range2: (cap[4].parse::<usize>().expect(""),cap[5].parse::<usize>().expect("")),
        }
    }).collect();
    let your_ticket: Vec<usize> = ticket_to_vec(input.lines().skip_while(|&l| l != "your ticket:").skip(1).next().unwrap());
    let nearby_tickets: Vec<Vec<usize>> = input.lines().skip_while(|&l| l != "nearby tickets:").skip(1).map(|l| ticket_to_vec(l)).collect();

    let p1 = nearby_tickets.iter().fold(0, |e, t| {
        e + t.iter().filter(|&value| {
            !ranges.iter().any(|r| r.valid_for(*value))
        }).sum::<usize>()
    });
    
    let valid_nearby_tickets: Vec<Vec<usize>> = nearby_tickets.iter().filter(|nearby_ticket| {
        nearby_ticket.iter().all(|&value| {
            ranges.iter().any(|r| r.valid_for(value))
        })
    }).cloned().collect();
    
    let mut unassigned: Vec<usize> = (0..ranges.len()).collect();
    let mut unknown_indices: Vec<usize> = (0..your_ticket.len()).collect();

    let mut assigned: HashMap<usize,usize> = HashMap::new();
    while unassigned.len() > 0 {
        let valid_fields_for_each_index: Vec<Vec<usize>> = unknown_indices.iter().map(|&index| { 
            unassigned.iter().filter(|&i| {
                valid_nearby_tickets.iter().all(|nt| ranges[*i].valid_for(nt[index]))
            }).cloned().collect()
        }).collect();

        for (i,vf) in valid_fields_for_each_index.iter().enumerate().filter(|(_,v)| v.len() == 1).rev() {
            let known = unknown_indices[i];

            let j = unassigned.iter().position(|&u| u == vf[0]).unwrap();
            unassigned.remove(j);
            assigned.insert(known, vf[0]);
            unknown_indices.remove(i);
        }
    }
    let p2 = assigned.iter().fold(1, |acc, (field_num, checker_index)| {
        if ranges[*checker_index].name.starts_with("departure") {
            acc * your_ticket[*field_num]
        } else {
            acc
        }
    });

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}
