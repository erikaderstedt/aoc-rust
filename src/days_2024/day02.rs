// https://adventofcode.com/2024/day/2

use crate::common::Solution;

fn is_report_safe(report: &Vec<isize>) -> bool {
    report
        .iter()
        .skip(1)
        .fold((report[0], true), |(previous, ok), value| {
            (*value, ok && value - previous >= 1 && value - previous <= 3)
        })
        .1
        || report
            .iter()
            .skip(1)
            .fold((report[0], true), |(previous, ok), value| {
                (*value, ok && previous - value >= 1 && previous - value <= 3)
            })
            .1
}

pub fn solve(input: &str) -> Solution {
    let reports: Vec<Vec<isize>> = input
        .lines()
        .map(|line| -> Vec<isize> { line.split(" ").flat_map(|s| s.parse::<isize>()).collect() })
        .collect();

    let p1 = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    let p2 = reports
        .iter()
        .filter(|report: &&Vec<isize>| -> bool {
            (0..report.len()).any(|i| -> bool {
                let mut r: Vec<isize> = report.to_vec().clone();
                r.remove(i);
                is_report_safe(&r)
            })
        })
        .count();

    Solution::new(p1, p2)
}
