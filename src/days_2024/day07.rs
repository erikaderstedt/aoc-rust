// https://adventofcode.com/2024/day/7

use crate::common::{parsed_from_each_line, Solution};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    result: usize,
    items: Vec<usize>,
}

impl Report {
    fn possible_results<F>(v: &[usize], mapper: &F) -> Vec<usize>
    where
        F: Fn(usize, usize) -> Vec<usize>,
    {
        if v.len() == 2 {
            mapper(v[0], v[1])
        } else {
            let last = v[v.len() - 1];
            Self::possible_results(&v[0..(v.len() - 1)], mapper)
                .into_iter()
                .flat_map(|p| mapper(p, last))
                .collect()
        }
    }
}

fn concatenate(x: usize, y: usize) -> usize {
    x * 10usize.pow(y.to_string().len() as u32) + y
}

pub fn solve(input: &str) -> Solution {
    let reports: Vec<Report> = parsed_from_each_line(input);

    let (possible, not_possible): (Vec<Report>, Vec<Report>) = reports
        .into_iter()
        .partition(|report| {
            Report::possible_results(&report.items, &|a, b| -> Vec<usize> { vec![a + b, a * b] })
                .contains(&report.result)
        });

    let p1 = possible.into_iter().map(|r| r.result).sum::<usize>();

    let p2 = p1 + not_possible
        .iter()
        .filter(|report| {
            Report::possible_results(&report.items, &|a, b| -> Vec<usize> { vec![a + b, a * b, concatenate(a, b)] })
                .contains(&report.result)
        })
        .map(|report| report.result)
        .sum::<usize>();

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
