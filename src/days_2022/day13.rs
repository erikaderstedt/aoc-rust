// https://adventofcode.com/2022/day/13

use crate::common::Solution;
use std::str::FromStr;
use std::cmp::Ordering;

const OPENING_BRACE: u8 = '[' as u8;
const CLOSING_BRACE: u8 = ']' as u8;
const COMMA: u8 = ',' as u8;

#[derive(Debug,Eq,PartialEq,Clone)]
enum Packet {
    Int(u32),
    List(Vec<Self>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(_)) => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (Packet::List(_), Packet::Int(b)) => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
            (Packet::List(a), Packet::List(b)) => {
                let mut i = 0;
                while i < a.len() && i < b.len() {
                    match a[i].cmp(&b[i]) {
                        Ordering::Equal => { i += 1; },
                        x => return x,
                    }
                }
                if i < a.len() { 
                    Ordering::Greater
                } else if i < b.len() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(input: &str) -> Solution {

    let packets: Vec<Packet> = input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<Packet>().unwrap())
        .collect();

    let p1: usize = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, items)| items[0].cmp(&items[1]) == Ordering::Less)
        .map(|p| p.0 + 1)
        .sum();

    let divider_packet1 = "[[2]]".parse::<Packet>().unwrap();
    let divider_packet2 = "[[6]]".parse::<Packet>().unwrap();

    let p2 = (packets.iter().filter(|p| (*p).cmp(&divider_packet1) == Ordering::Less).count() + 1) *
             (packets.iter().filter(|p| (*p).cmp(&divider_packet2) == Ordering::Less).count() + 2);

    Solution::new(p1, p2)
}

impl FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b[0] != OPENING_BRACE {
            let n = s.parse::<u32>().map_err(|_| "Invalid integer literal")?;
            Ok(Packet::Int(n))
        } else {
            if b[1] == CLOSING_BRACE {
                Ok(Packet::List(vec![]))
            } else {
                Ok(Packet::List(
                    CommaSeparatedIterator::new(&b)
                        .map(|(start,end)| std::str::from_utf8(&b[start..end]).unwrap().parse::<Packet>())
                        .collect::<Result<Vec<Packet>,Self::Err>>()
                        .map_err(|_| "Unable to parse packet list.")?                        
                    ))
            }
        }
    }
}

pub struct CommaSeparatedIterator<'a> {
    previous: usize,
    index: usize,
    brace_level: i32,
    byte_array: &'a [u8],
}

impl<'a> CommaSeparatedIterator<'a> {
    fn new(s: &'a [u8]) -> Self {
        debug_assert!(s[0] == OPENING_BRACE, "Array must begin with an opening brace.");
        debug_assert!(s[s.len()-1] == CLOSING_BRACE, "Array must end with a closing brace.");
        Self { previous: 1, index: 1, brace_level: 0 , byte_array: s }
    }
}

impl<'a> Iterator for CommaSeparatedIterator<'a> {
    type Item = (usize,usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.byte_array.len() - 1 {
            match self.byte_array[self.index] {
                OPENING_BRACE => { self.brace_level += 1 },
                CLOSING_BRACE => { self.brace_level -= 1; },
                COMMA => if self.brace_level == 0 { 
                    let r = (self.previous, self.index);
                    self.index += 1;
                    self.previous = self.index;
                    return Some(r) 
                },
                _ => {},
            };
            self.index += 1;
        }
        if self.previous != self.index {
            let r = (self.previous, self.index);
            self.previous = self.index;
            Some(r)
        } else { None }
    }
}
