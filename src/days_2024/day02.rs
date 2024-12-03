// https://adventofcode.com/2024/day/2

use itertools::Itertools;

use crate::common::Solution;

fn is_report_safe(report: &[isize]) -> bool {
    let (_, ascending, descending) =
        report
            .iter()
            .skip(1)
            .fold((report[0], true, true), |(previous, ok_a, ok_d), value| {
                let diff = value - previous;
                (
                    *value,
                    ok_a && diff >= 1 && diff <= 3,
                    ok_d && diff >= -3 && diff <= -1,
                )
            });

    ascending || descending
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
            report
                .iter()
                .cloned()
                .combinations(report.len() - 1)
                .any(|v| is_report_safe(&v[..]))
        })
        .count();

    Solution::new(p1, p2)
}
