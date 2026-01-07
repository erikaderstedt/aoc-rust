// https://adventofcode.com/2015/day/12

use crate::common::Solution;

const BRACE: u8 = '[' as u8;
const QUOTE: u8 = '"' as u8;
const END_BRACE: u8 = ']' as u8;
const CURLY_BRACE: u8 = '{' as u8;
const COMMA: u8 = ',' as u8;
const COLON: u8 = ':' as u8;
const END_CURLY_BRACE: u8 = '}' as u8;
const MINUS: u8 = '-' as u8;

enum JsonResult {
    QuotedString(String),
    Value(i64),
}
fn parse_number(s: &[u8], i: &mut usize) -> i64 {
    let mut j = i.clone();
    while s[j].is_ascii_digit() || s[j] == MINUS {
        j += 1;
    }
    let value = std::str::from_utf8(&s[(*i)..j])
        .unwrap()
        .parse::<i64>()
        .unwrap();
    *i = j;
    value
}

fn parse_value<const IGNORE_RED: bool>(s: &[u8], i: &mut usize) -> JsonResult {
    match s[*i] {
        BRACE => JsonResult::Value(parse_array::<IGNORE_RED>(s, i)),
        CURLY_BRACE => JsonResult::Value(parse_object::<IGNORE_RED>(s, i)),
        QUOTE => parse_str(s, i),
        x if x.is_ascii_digit() => JsonResult::Value(parse_number(s, i)),
        MINUS => JsonResult::Value(parse_number(s, i)),
        a => panic!("?: {}", a as char),
    }
}

fn parse_object<const IGNORE_RED: bool>(s: &[u8], i: &mut usize) -> i64 {
    let mut sum = 0;
    let mut factor = 1;
    // Parse quoted string, then colon, then a value.
    // if next is comma, repeat
    // otherwise assert that it is END_CURLY_BRACE
    *i += 1;
    loop {
        match parse_value::<IGNORE_RED>(s, i) {
            JsonResult::QuotedString(_) => {}
            JsonResult::Value(_) => panic!("Key expected!"),
        };

        assert!(s[*i] == COLON);
        *i += 1;

        match parse_value::<IGNORE_RED>(s, i) {
            JsonResult::Value(x) => {
                sum += x;
            }
            JsonResult::QuotedString(v) => {
                if v == "red" && IGNORE_RED {
                    factor = 0;
                }
            }
        };

        if s[*i] == END_CURLY_BRACE {
            *i += 1;
            break;
        }
        assert!(s[*i] == COMMA);
        *i += 1;
    }

    sum * factor
}

fn parse_array<const IGNORE_RED: bool>(s: &[u8], i: &mut usize) -> i64 {
    let mut sum = 0;

    *i += 1;
    loop {
        match parse_value::<IGNORE_RED>(s, i) {
            JsonResult::QuotedString(_) => {}
            JsonResult::Value(x) => {
                sum += x;
            }
        }
        if s[*i] == END_BRACE {
            *i += 1;
            break;
        }
        assert!(s[*i] == COMMA);
        *i += 1;
    }

    sum
}

fn parse_str(s: &[u8], i: &mut usize) -> JsonResult {
    let start = i.clone();
    let j = s[(start + 1)..].iter().position(|v| *v == QUOTE).unwrap() + start + 1;
    *i = j + 1;
    JsonResult::QuotedString(std::str::from_utf8(&s[(start + 1)..j]).unwrap().to_string())
}

pub fn solve(input: &str) -> Solution {
    let mut i = 0;
    let p1 = parse_object::<false>(input.as_bytes(), &mut i);

    i = 0;
    let p2 = parse_object::<true>(input.as_bytes(), &mut i);

    Solution::new(p1, p2)
}
