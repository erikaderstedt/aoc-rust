use crate::common::Solution;
use std::io::{BufRead,Read,BufReader,Cursor};

const OPENING_PARENTHESES: u8 = '(' as u8;
const CLOSING_PARENTHESES: u8 = ')' as u8;
const TIMES: u8 = '*' as u8;
const PLUS: u8 = '+' as u8;

fn evaluate_p1(v: Vec<u8>) -> std::io::Result<i64> {
    let mut r = BufReader::new(Cursor::new(v));
    let mut result = get_token(&mut r, evaluate_p1)?;    
    let mut first = vec![0u8,0u8];

    while matches!(r.read_exact(&mut first), Ok(())) {
        let t2 = get_token(&mut r, evaluate_p1)?;
        result = match first[0] {
            TIMES => result * t2,
            PLUS => result + t2,
            _ => {panic!("Invalid operator {:?}", first); }
        };
    }
    Ok(result)
}

fn evaluate_p2(v: Vec<u8>) -> std::io::Result<i64> {
    // Get a list of tokens and operators
    let mut r = BufReader::new(Cursor::new(v));
    let mut values: Vec<i64> = vec![get_token(&mut r, evaluate_p2)?];
    let mut operators: Vec<u8> = Vec::new();
    let mut first = vec![0u8,0u8];

    while match r.read_exact(&mut first) { Ok(()) => true, _ => false } {
        values.push(get_token(&mut r, evaluate_p2)?);
        operators.push(first[0]);
    }
    for (i, _) in operators.into_iter().enumerate().rev().filter(|(_,op)| *op == PLUS) {
        let v1 = values.remove(i+1);
        let v2 = values[i];
        values[i] = v1 + v2;
    }
    Ok(values.into_iter().product())
}

fn is_balanced(v: &Vec<u8>) -> bool {
    let no = v.iter().filter(|&u| *u == OPENING_PARENTHESES).count();
    let nc = v.iter().filter(|&u| *u == CLOSING_PARENTHESES).count();
    no == nc
}

fn get_token<R: Read+BufRead, F>(r: &mut R, evaluator: F) -> std::io::Result<i64> 
    where F: Fn(Vec<u8>) -> std::io::Result<i64> {
    let mut first = vec![0u8];

    r.read_exact(&mut first)?;
    match first[0] {
        OPENING_PARENTHESES => {
            while !is_balanced(&first) { r.read_until(CLOSING_PARENTHESES, &mut first)?; }
            if r.read_until(' ' as u8, &mut first)? > 0 { first.pop(); }
            let content = first[1..(first.len()-1)].to_vec();
            evaluator(content)
        },
        _ => { 
            if matches!(r.read_until(' ' as u8, &mut first), Ok(n) if n > 0) { first.pop(); }
            Ok(std::str::from_utf8(&first).unwrap().parse::<i64>().unwrap())
        },
    }
}

pub fn solve(input: &str) -> Solution {
    let p1 = input.lines().map(|line| -> i64 {
        evaluate_p1(line.as_bytes().to_vec()).expect("Bad string")
    }).sum::<i64>();
    let p2 = input.lines().map(|line| -> i64 {
        evaluate_p2(line.as_bytes().to_vec()).expect("Bad string")
    }).sum::<i64>();
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}