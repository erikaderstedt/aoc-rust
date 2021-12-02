use crate::common::Solution;
use serde_json::{Value,from_str};

fn numbers(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64(),
        Value::Null | Value::String(_) | Value::Bool(_) => None,
        Value::Array(a) => Some(a.iter().filter_map(|x| numbers(x)).sum()),
        Value::Object(h) => Some(h.iter().filter_map(|(_k,v)| numbers(v)).sum()),
    }
}

fn object_has_red_as_value_for_any_property(v: &Value) -> bool {
    if let Value::Object(h) = v {
        h.values().filter(|x| match x { Value::String(s) if s == "red" => true, _ => false })                
                .count() > 0
    } else {
        panic!("You promised!");
    }
}

fn numbers_except_red(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64(),
        Value::Array(a) => Some(a.iter().filter_map(|x| numbers_except_red(x)).sum()),
        Value::Object(h) if !object_has_red_as_value_for_any_property(&v) => Some(h.iter().filter_map(|(_k,v)| numbers_except_red(v)).sum()),
        _ => None,
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let v: Value = from_str(&lines.join("\n")).expect("Inavlid JSON");

    let p1 = numbers(&v).unwrap();
    let p2 = numbers_except_red(&v).unwrap();

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}