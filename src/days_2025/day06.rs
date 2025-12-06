// https://adventofcode.com/2025/day/6

use itertools::{Itertools, rev};

use crate::common::Solution;

#[derive(Debug)]
enum Operator {
    Multiply,
    Add
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn solve(input: &str) -> Solution {

    let n = input.lines().count();
    let numbers: Vec<Vec<i64>> = input.lines().take(n-1).map(|line|
        line.split_whitespace().map(|r| r.parse::<i64>().unwrap()).collect()
    ).collect();
    let operators: Vec<Operator> = input.lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|r| {
            match r {
                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => panic!("Bad operator")
            }
        }).collect();

    let numbers = transpose2(numbers);

    let p1 = operators.iter().enumerate()
        .fold(0, |acc, (index, operator)| {
            let numbers = &numbers[index];
        acc + match operator {
            Operator::Multiply => numbers.iter().fold(1, |acc, value| acc*value),
            Operator::Add => numbers.iter().fold(0, |acc, value| acc + value),
        }
        });

    // Find space positions
    // all spaces in row 0, row 1 ..,. Append them, sort them, and find which has n occurrences.
    
    let mut spaces: Vec<usize> =
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == ' ')
                .map(|(i,_)| i)
        })
        .flatten()
        .sorted()
        .dedup_with_count()
        .filter(|(count, _)| *count >= n)
        .map(|(_, i)| i)
        .collect();

    spaces.insert(0, 0);
    spaces.push(input.lines().next().unwrap().len());

    let p2: u64 = spaces
        .iter()
        .zip(spaces.iter().skip(1))
        .map(|(start, stop)| {
            let numbers: Vec<u64> = ((*start)..(*stop))
                .flat_map(|column| {
                    let digits: Vec<String> = input
                        .lines()
                        .take(n-1)
                        .map(|line| {
                            line[column..(column+1)].to_string()

                        })
                        .collect();
                    let s = digits
                        .join("")
                        .trim_end()
                        .replace(" ", "0");

                        s.parse::<u64>().ok()
                })
                .collect();

            let operator = match input.lines().last().unwrap()[(*start)..(*stop)].trim() {
                                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => panic!("Bad operator")
            };

            match operator {
            Operator::Multiply => numbers.iter().fold(1, |acc, value| acc*value),
            Operator::Add => numbers.iter().fold(0, |acc, value| acc + value),
        }

        })
        .sum::<u64>();
    

    
    Solution::new(p1, p2)
}

// 4352465490574 wrong
