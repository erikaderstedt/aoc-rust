// https://adventofcode.com/2021/day/7
use crate::common::Solution;
use std::collections::HashSet;
use itertools::Itertools;
// use std::str::FromStr;

// 0 : 6 segment
// 1 : 2 segment
// 2: 5, 3: 5, 4:4 5:5 6:6 7:3 8:8 9:6

// enum Movement {
//     Down(i64),
//     Up(i64),
//     Forward(i64),
// }

// impl FromStr for Movement {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.split_once(" | ") {
//             Some((_first, second)) => {
//                 second.split(' ').
//                 let x = magnitude.parse::<i64>().map_err(|_| "Invalid integer literal")?;
//                 match command {
//                     "down" => Ok(Movement::Down(x)),
//                     "up" => Ok(Movement::Up(x)),
//                     "forward" => Ok(Movement::Forward(x)),
//                     _ => Err("Bad instruction"),
//                 }},
//             _ => Err("Malformed line."),
//         }
//     }
// }

// At the beginning, each segment can be any letter.
// if 1
//     if 1 and 7 -> top is known.

fn segments(s: &str) -> u8 {
    let mut x = 0u8;
    for c in s.chars() {
        match c {
            'a' => x += 1 << 0,
            'b' => x += 1 << 1,
            'c' => x += 1 << 2,
            'd' => x += 1 << 3,
            'e' => x += 1 << 4,
            'f' => x += 1 << 5,
            'g' => x += 1 << 6,
            _ => {},
        }
    }
    x
}

pub fn solve(input: &str) -> Solution {
    let mut m1 = 0;
    for line in input.lines() {
        m1 += match line.split_once(" | ") {
            Some((_first, second)) => {
                second.split(' ').filter(|s| 
                    match s.len() {
                        2 | 4 | 3 | 7 => { true }
                        _ => { false },
                    }).count()
            },
            _ => { 0 },            
        };
    }
    let mut m2 = 0;
    for line in input.lines().skip(3) {
        m2 += match line.split_once(" | ") {
            Some((first, second)) => {
                //  0000
                // 1    2
                // 1    2
                //  3333
                // 4    5
                // 4    5
                //  6666
                // Keep u8 for all possible locations for a signal
                // If we have a 6 and have already received a different 6 (different bits),
                // then and them together and get 0,1,3,5,6

                let mut x = [127u8; 7];
                let mut h5: HashSet<u8> = HashSet::new();
                let mut h6: HashSet<u8> = HashSet::new();
                let mut f1: Option<u8> = None;
                let mut f7: Option<u8> = None;
                
                for s in first.split(' ').sorted_by(|s1,s2| s1.len().cmp(&s2.len())) {
                    let segs = segments(s);
                    match s.len() {
                        2 => { 
                            x[2] = segs;
                            x[5] = segs;
                            f1 = Some(segs);
                        },
                        3 => {
                            match f1 {
                                Some(o) => { x[0] = segs ^ o;},
                                None => {
                                    x[0] = segs;
                                    x[2] = segs;
                                    x[5] = segs;
                                }
                            };
                            f7 = Some(segs);
                        },
                        4 => {
                            match f1 {
                                Some(o) => {
                                    x[1] = segs ^ o;
                                    x[3] = segs ^ o;
                                }
                                None => { 
                                    x[1] &= segs;
                                    x[2] &= segs;
                                    x[3] &= segs;
                                    x[5] &= segs;
                                }
                            }
                        },
                        6 => { // 0, 6, 9
                            h6.insert(segs);
                            if h6.len() == 2 {
                                let segs2 = h6.iter().fold(255u8,|x, v| x & (*v));
                                x[0] &= segs2;
                                x[1] &= segs2;
                                x[5] &= segs2;
                                x[6] &= segs2;
                            }
                        },
                        5 => {
                            h5.insert(segs);
                            if h5.len() == 3 {
                                let segs2 = h5.iter().fold(255u8,|x, v| x & (*v));
                                x[0] &= segs2;
                                x[3] &= segs2;
                                x[6] &= segs2;
                            }
                        },
                        _ => {},
                    }
                    println!("{} {} {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}", s, s.len(), x[0], x[1], x[2], x[3], x[4], x[5], x[6]);
                }

                if x.iter().all(|q| q.count_ones() > 1) {
                    panic!("No solution: {}", line);
                }
                
                // Any with just a single bit is now determined
                // Determine bit -> segment
                let mut y = [0u8;7];
                while x.iter().any(|q| q.count_ones() >= 1) {
                    // Find first with 1
                    let (segment_index, bit_value): (usize, u8) = x.iter().enumerate().find(|(i,v)| v.count_ones() == 1).map(|(i,v)| (i, v.clone())).unwrap();
                    y[segment_index] = bit_value;
                    for i in 0..7 { x[i] &= !bit_value }
                    println!("x: {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}",  x[0], x[1], x[2], x[3], x[4], x[5], x[6]);
                    println!("y: {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}",  y[0], y[1], y[2], y[3], y[4], y[5], y[6]);
       
                }

    // println!("Final: {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}",  y[0], y[1], y[2], y[3], y[4], y[5], y[6]);
                
                let digits = [
                    y[0] | y[1] | y[2] | y[4] | y[5] | y[6],
                    y[2] | y[5],
                    y[0] | y[2] | y[3] | y[4] | y[6],
                    y[0] | y[2] | y[3] | y[5] | y[6],
                    y[1] | y[2] | y[3] | y[5],
                    y[0] | y[2] | y[3] | y[5] | y[6],
                    y[0] | y[1] | y[3] | y[4] | y[5] | y[6],
                    y[0] | y[2] | y[5],
                    y[0] | y[1] | y[2] | y[3] | y[4] | y[5] | y[6],
                    y[0] | y[1] | y[2] | y[3] | y[5] | y[6]];

                let mut num = 0;
                for a in second.split(' ') {
                    let q = segments(a);
                    // println!("{:>07b} ||| {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b}  {:>07b} {:>07b}  {:>07b}  {:>07b}", q, digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6], digits[7], digits[8], digits[9]);
                    let d = digits.iter().enumerate().find(|(_i,&d)| d == q).unwrap().0;
                    num = 10*num + d;
                }

                num
            },
            _ => { 0 },            
        };
    }
    Solution::new(m1, m2)
}
