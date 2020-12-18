use crate::common::Solution;
extern crate regex;
use regex::{Regex, Captures};

fn evaluate_expression(exp: &str) -> i64 {
//    println!("evaluatig {}", exp);
    let mut digits = exp.split(|c| c == '*' || c == '+').map(|c| c.trim().parse::<i64>().unwrap());
    let operators = exp.chars().filter(|&c| c == '*' || c == '+');
    let mut result = digits.next().unwrap();
    for (digit, operator) in digits.zip(operators) {
        match operator {
            '*' => result *= digit,
            '+' => result += digit,
            _ => { },
        }
    }
    result
}

fn evaluate_expression_p2(exp: &str) -> i64 {
    let r = Regex::new(r"(\d+) \+ (\d+)").expect("akldf");
    let r2 = Regex::new(r"(\d+) \* (\d+)").expect("akldf");
    let mut e2 = exp.to_string();
    
    loop {
        println!("e2+ {}", e2);
        let e3 = r.replace_all(&e2, |caps: &Captures| {
            let i1: i64 = caps[1].parse().unwrap();
            let i2: i64 = caps[2].parse().unwrap();
            (i1+i2).to_string()
        });
        if e2 == e3 { break }
        e2 = e3.to_string();
    }

    loop {
        println!("e2* {}", e2);
        let e3 = r2.replace_all(&e2, |caps: &Captures| {
            let i1: i64 = caps[1].parse().unwrap();
            let i2: i64 = caps[2].parse().unwrap();
            (i1*i2).to_string()
        });
        if e2 == e3 { break }
        e2 = e3.to_string();
    }

    e2.parse::<i64>().unwrap()
}

pub fn solve(input: &str) -> Solution {
    let r = Regex::new(r"\(([^)()]+)\)").expect("bad");

    let p1: i64 = input.lines().map(|line| -> i64 {
        let mut s = line.to_string();
        loop {
            let s2 = r.replace_all(&s, |caps: &Captures| {
                let j = evaluate_expression(&caps[1]);
                j.to_string()
            });
            if s == s2 { break }
            s = s2.to_string();
        }
        assert!(!s.contains("("));
        assert!(!s.contains(")"));
        evaluate_expression(&s)
    }).sum();

    let p2: i64 = input.lines().map(|line| -> i64 {
        let mut s = line.to_string();
        loop {
            let s2 = r.replace_all(&s, |caps: &Captures| {
                let j = evaluate_expression_p2(&caps[1]);
                j.to_string()
            });
            if s == s2 { break }
            s = s2.to_string();
        }
        assert!(!s.contains("("));
        assert!(!s.contains(")"));
        evaluate_expression_p2(&s)
    }).sum();


// 620691361 wrong
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}