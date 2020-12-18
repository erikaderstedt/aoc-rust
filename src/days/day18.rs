use crate::common::Solution;

const OPENING_PARENTHESES: u8 = '(' as u8;
const CLOSING_PARENTHESES: u8 = ')' as u8;
const TIMES: u8 = '*' as u8;
const PLUS: u8 = '+' as u8;

fn evaluate_p1(s: &mut &[u8]) -> i64 {
    let mut result = get_token(s, evaluate_p1);    

    while s.len() > 0 {
        // Get operator
        let operator = s[0];
        *s = &s[2..];
        let t2 = get_token(s, evaluate_p1);
        result = match operator {
            TIMES => result * t2,
            PLUS => result + t2,
            _ => panic!("Unknown operator {}", operator),
        }
    }
    result
}

fn evaluate_p2(s: &mut &[u8]) -> i64 {
    let mut values: Vec<i64> = vec![get_token(s, evaluate_p2)];
    let mut operators: Vec<u8> = Vec::new();

    while s.len() > 0 {
        operators.push(s[0]);
        *s = &s[2..];
        values.push(get_token(s, evaluate_p2));
    }
    for (i, _) in operators.into_iter().enumerate().rev().filter(|(_,op)| *op == PLUS) {
        let v1 = values.remove(i+1);
        let v2 = values[i];
        values[i] = v1 + v2;
    }
    values.into_iter().product()
}

fn get_token<F>(s: &mut &[u8], evaluator: F) -> i64 
    where F: Fn(&mut &[u8]) -> i64 {
    match s[0] {
        OPENING_PARENTHESES => {
            let closing_parentheses_location = (1..s.len()).scan(1, |number_of_open_parentheses, i| { 
                match s[i] {
                    OPENING_PARENTHESES => { *number_of_open_parentheses = *number_of_open_parentheses + 1; },
                    CLOSING_PARENTHESES => { *number_of_open_parentheses = *number_of_open_parentheses - 1; },
                    _ => {},
                }
                if *number_of_open_parentheses > 0 { Some(i) } else { None }
            }).last().unwrap()+1;
            let n = evaluator(&mut &s[1..closing_parentheses_location]);
            *s = &s[(closing_parentheses_location+1)..];
            if s.len() > 0 { *s = &s[1..]; }
            n
        },
        digit => {
            *s = &s[(if s.len() > 1 { 2 } else { 1 })..];
            (digit - 48) as i64
        },
    }
}

pub fn solve(input: &str) -> Solution {
    let p1 = input.lines().map(|line| evaluate_p1(&mut line.as_bytes())).sum::<i64>();
    let p2 = input.lines().map(|line| evaluate_p2(&mut line.as_bytes())).sum::<i64>();
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}