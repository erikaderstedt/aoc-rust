// https://adventofcode.com/2021/day/24
use crate::common::Solution;

enum Direction {
    Push,
    Pop,
}

struct Operation {
    direction: Direction,
    add: isize,
}

pub fn solve(input: &str) -> Solution {
    let divides: Vec<isize> = input.lines().skip(4).step_by(18).map(|s| s.split(' ').skip(2).next().unwrap().parse::<isize>().unwrap()).collect();
    let add1s: Vec<isize> = input.lines().skip(5).step_by(18).map(|s| s.split(' ').skip(2).next().unwrap().parse::<isize>().unwrap()).collect();
    let add2s: Vec<isize> = input.lines().skip(15).step_by(18).map(|s| s.split(' ').skip(2).next().unwrap().parse::<isize>().unwrap()).collect();

    let operations: Vec<Operation> = divides.into_iter().zip(add1s.into_iter()).zip(add2s.into_iter()).map(|((d,a1),a2)| 
        if d == 26 { 
            Operation { direction: Direction::Pop, add: a1 }
        } else {
            Operation { direction: Direction::Push, add: a2 }
        }).collect();

    let mut m1 = [0isize;14];
    let mut m2 = [0isize;14];
    let mut s: Vec<(usize,Operation)> = Vec::new();
    for (index,operation) in operations.into_iter().enumerate() {
        match operation.direction {
            Direction::Pop => {
                let (prev_index, previous) = s.pop().unwrap();
                let delta = previous.add + operation.add;
                if delta > 0 {
                    m1[index] = 9;
                    m1[prev_index] = 9 - delta;
                    m2[prev_index] = 1;
                    m2[index] = 1 + delta;
                } else {
                    m1[prev_index] = 9;
                    m1[index] = 9 + delta;
                    m2[index] = 1;
                    m2[prev_index] = 1 - delta;
                }},
            Direction::Push => {
                s.push((index, operation))
            },
        }
    }
    let m1 = m1.iter().fold(0, |s, x| 10*s + x);
    let m2 = m2.iter().fold(0, |s, x| 10*s + x);

    Solution::new(m1, m2)
}

// impl Operation {

//     fn operate(&self, z: isize, w: isize) -> isize {
//         let x = (z % 26) + self.first_add;
//         let mut z2 = z / self.divide;
//         if x != w {
//             z2 *= 26;
//             z2 += w + self.second_add;
//         }
//         // z should be zero after the last operation.
//         z2
//     }
// }
