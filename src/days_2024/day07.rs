// https://adventofcode.com/2024/day/7

use crate::common::{parsed_from_each_line, Solution};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    result: usize,
    items: Vec<usize>,
}

impl Report {
    fn check<const ALLOW_CONCAT: bool>(items: &[usize], target: usize) -> bool {
        let last = items[items.len() - 1];
        if items.len() == 1 {
            target == last
        } else {
            let remainder = &items[0..items.len() - 1];
            (target % last == 0
                && Self::check::<ALLOW_CONCAT>(remainder, target / last))
                || (target > last
                    && Self::check::<ALLOW_CONCAT>(remainder, target - last))
                || (ALLOW_CONCAT
                    && target > last
                    && match target.to_string().strip_suffix(&last.to_string()) {
                        Some(reduced) => Self::check::<ALLOW_CONCAT>(remainder,reduced.parse::<usize>().unwrap()),
                        None => false,
                    })
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let reports: Vec<Report> = parsed_from_each_line(input);

    let p1: usize = reports
        .iter()
        .filter(|report| Report::check::<false>(&report.items, report.result))
        .map(|report| report.result)
        .sum();
    let p2: usize = reports
        .iter()
        .filter(|report| Report::check::<true>(&report.items, report.result))
        .map(|report| report.result)
        .sum();

    Solution::new(p1, p2)
}

impl FromStr for Report {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (
            s.split(':').next().and_then(|x| x.parse::<usize>().ok()),
            s.split(' ')
                .skip(1)
                .filter_map(|i| i.parse::<usize>().ok())
                .collect::<Vec<usize>>(),
        ) {
            (Some(result), items) => Ok(Report { result, items }),
            _ => Err("unable to parse"),
        }
    }
}
