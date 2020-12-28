use crate::common::Solution;
use itertools::Itertools;
use std::collections::{HashMap,HashSet};

struct Shift {
    guard: usize,
    asleep: [usize;60],
}

pub fn solve(input: &str) -> Solution { 
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    let shifts:Vec<Shift> = lines.join("\n").split("Guard #").skip(1).map(|shift| -> Shift {
        let i = shift.find(' ').unwrap();
        let guard = shift[..i].parse::<usize>().unwrap();
        let mut asleep = [0;60];
        for (falls_asleep,wakes_up) in shift.lines().skip(1).tuples() {
            let f = falls_asleep[15..17].parse::<usize>().unwrap();
            let w = wakes_up[15..17].parse::<usize>().unwrap();
            for n in f..w { asleep[n] = 1; }
        }
        Shift { guard, asleep }
    }).collect();

    let guards: HashSet<usize> = shifts.iter().map(|shift| shift.guard).collect();
    
    let sleep_amounts: HashMap<usize, [usize;60]> = guards.iter().map(|guard_id| {
        (guard_id.clone(), shifts.iter()
            .filter(|shift| shift.guard == *guard_id)
            .fold([0;60], |mut acc, shift| {
            for n in 0..60 {
                acc[n] += shift.asleep[n];
            }
            acc
        }))
    }).collect();

    let guard_who_is_most_asleep = sleep_amounts.iter().max_by(|g1,g2| {
        g1.1.iter().sum::<usize>().cmp(&g2.1.iter().sum::<usize>())
    }).unwrap().0;
    let best_minute = sleep_amounts[guard_who_is_most_asleep].iter().enumerate().max_by(|m1,m2| m1.1.cmp(&m2.1)).unwrap().0;

    let p1 = guard_who_is_most_asleep * best_minute;

    let most_slept_minute = sleep_amounts.iter().max_by(|g1, g2| {
        let most_frequent_guard_1 = g1.1.iter().max().unwrap();
        let most_frequent_guard_2 = g2.1.iter().max().unwrap();
        most_frequent_guard_1.cmp(&most_frequent_guard_2)
    }).unwrap();
    let p2 = most_slept_minute.0 * most_slept_minute.1.iter().enumerate().max_by(|m1,m2| m1.1.cmp(&m2.1)).unwrap().0;

    Solution::new(p1, p2)
}
