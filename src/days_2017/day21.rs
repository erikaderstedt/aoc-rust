// https://adventofcode.com/2017/day/21

use crate::common::Solution;
use itertools::Itertools;
use std::{collections::HashMap, convert::TryInto};

type Pattern3 = [u8; 3];
type Pattern2 = [u8; 2];
type Pattern4 = [u8; 4];

fn flip_up_down<const N: usize>(p: &[u8; N]) -> [u8; N] {
    let mut out = p.clone();
    for i in 0..(N / 2) {
        let s = out[i];
        out[i] = out[N - 1 - i];
        out[N - 1 - i] = s;
    }
    out
}

fn rotate_90<const N: usize>(p: &[u8; N]) -> [u8; N] {
    let mut out = [0; N];
    for i in 0..N {
        for j in 0..N {
            out[j] |= ((p[i] >> j) & 1) << (N - 1 - i);
        }
    }
    out
}

fn parse<const N: usize>(s: &str) -> [u8; N] {
    s.split('/')
        .map(|x| {
            x.chars().enumerate().fold(0, |a, (i, c)| {
                a | if c == '#' { 1 << (N - 1 - i) } else { 0 }
            }) as u8
        })
        .collect_array()
        .unwrap()
}

fn expand<const M: usize, const N: usize>(
    left: [u8; M],
    right: [u8; N],
) -> Vec<([u8; M], [u8; N])> {
    let r90 = rotate_90(&left);
    let r180 = rotate_90(&r90);
    let r270 = rotate_90(&r180);

    let left = vec![
        flip_up_down(&r270),
        r270,
        flip_up_down(&r180),
        r180,
        flip_up_down(&r90),
        r90,
        flip_up_down(&left),
        left,
    ];
    let right = vec![right.clone(); 12];
    left.into_iter().zip(right.into_iter()).collect()
}

fn split(p4: &Pattern4) -> ([Pattern2; 2], [Pattern2; 2]) {
    (
        [[p4[0] >> 2, p4[1] >> 2], [p4[0] & 3, p4[1] & 3]],
        [[p4[2] >> 2, p4[3] >> 2], [p4[2] & 3, p4[3] & 3]],
    )
}

fn mix(top: &[Pattern3; 2], bot: &[Pattern3; 2]) -> ([Pattern2; 3], [Pattern2; 3], [Pattern2; 3]) {
    let t_l: Pattern2 = [top[0][0] >> 1, top[0][1] >> 1];
    let t_m: Pattern2 = [
        ((top[0][0] & 1) << 1) | (top[1][0] >> 2),
        ((top[0][1] & 1) << 1) | (top[1][1] >> 2),
    ];
    let t_r: Pattern2 = [top[1][0] & 3, top[1][1] & 3];
    let m_l: Pattern2 = [top[0][2] >> 1, bot[0][0] >> 1];
    let m_m: Pattern2 = [
        ((top[0][2] & 1) << 1) | (top[1][2] >> 2),
        ((bot[0][0] & 1) << 1) | (bot[1][0] >> 2),
    ];
    let m_r: Pattern2 = [top[1][2] & 3, bot[1][0] & 3];
    let b_l: Pattern2 = [bot[0][1] >> 1, bot[0][2] >> 1];
    let b_m: Pattern2 = [
        ((bot[0][1] & 1) << 1) | (bot[1][1] >> 2),
        ((bot[0][2] & 1) << 1) | (bot[1][2] >> 2),
    ];
    let b_r: Pattern2 = [bot[1][1] & 3, bot[1][2] & 3];
    ([t_l, t_m, t_r], [m_l, m_m, m_r], [b_l, b_m, b_r])
}

enum Grid {
    Two(Vec<Pattern2>),
    Three(Vec<Pattern3>),
}

impl Grid {
    fn count(&self) -> u32 {
        match self {
            Self::Three(v) => v
                .iter()
                .map(|v| v.iter().map(|q| q.count_ones()).sum::<u32>())
                .sum::<u32>(),
            Self::Two(v) => v
                .iter()
                .map(|v| v.iter().map(|q| q.count_ones()).sum::<u32>())
                .sum::<u32>(),
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let two_to_three: HashMap<Pattern2, Pattern3> = input
        .lines()
        .take(6)
        .map(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            let left: Pattern2 = parse(left);
            let right: Pattern3 = parse(right);
            expand(left, right).into_iter()
        })
        .flatten()
        .collect();
    let three_to_four: HashMap<Pattern3, Pattern4> = input
        .lines()
        .skip(6)
        .map(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            let left: Pattern3 = parse(left);
            let right: Pattern4 = parse(right);
            expand(left, right).into_iter()
        })
        .flatten()
        .collect();

    assert!(two_to_three.len() == 1 << 4);
    assert!(three_to_four.len() == 1 << 9);

    let mut grid = Grid::Three(vec![[0b010, 0b001, 0b111]]);

    let mut p1 = 0;
    for i in 0..18 {
        grid = match grid {
            Grid::Three(three_grid) => {
                let side = three_grid.len().isqrt();
                let mut two_grid = vec![];
                let mut lower_part = two_grid.clone();
                let mut n = 0;
                for p in three_grid.iter() {
                    let out = three_to_four.get(p).unwrap();
                    let (top, bottom) = split(out);
                    two_grid.extend(top);
                    lower_part.extend(bottom);
                    n += 1;
                    if n == side {
                        two_grid.extend(lower_part);
                        lower_part = vec![];
                        n = 0;
                    }
                }
                Grid::Two(two_grid)
            }
            Grid::Two(two_grid) => {
                let mut three_grid = vec![];
                for p in two_grid.iter() {
                    let out = two_to_three.get(p).unwrap().clone();
                    three_grid.push(out);
                }
                let side = three_grid.len().isqrt();

                if side & 1 == 0 {
                    // Convert to two-grid in preparation for the next iteration.
                    // Be sure to maintain reading order.
                    let mut two_grid = vec![];
                    for row in (0..side).step_by(2) {
                        let mut middle = vec![];
                        let mut bottom = vec![];
                        for n in (0..side).step_by(2) {
                            let (t, m, b) = mix(
                                three_grid[(side * row + n)..(side * row + n + 2)]
                                    .try_into()
                                    .unwrap(),
                                three_grid[(side * (row + 1) + n)..(side * (row + 1) + n + 2)]
                                    .try_into()
                                    .unwrap(),
                            );
                            two_grid.extend(t);
                            middle.extend(m);
                            bottom.extend(b);
                        }
                        two_grid.extend(middle);
                        two_grid.extend(bottom);
                    }

                    Grid::Two(two_grid)
                } else {
                    Grid::Three(three_grid)
                }
            }
        };

        if i == 4 {
            p1 = grid.count();
        }
    }

    let p2: u32 = grid.count();

    Solution::new(p1, p2)
}
