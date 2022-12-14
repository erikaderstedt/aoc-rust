// https://adventofcode.com/2022/day/14
use crate::common::Solution;

const X_OFFSET: usize = 300;
const HEIGHT: usize = 170;
const WIDTH: usize = 400;

// const X_OFFSET: usize = 480;
// const HEIGHT: usize = 20;
// const WIDTH: usize = 40;

const START_X: usize = 500 - X_OFFSET;
const START_Y: usize = 0;

type Cave = [u8; WIDTH*HEIGHT];
const AIR: u8 = 0;
const SAND: u8 = 1;
const ROCK: u8 = 2;

fn introduce_sand(cave: &mut Cave, mut x: usize, mut y: usize) -> bool {
    loop { 
        y += 1;
        if y == HEIGHT {
            return false;
        }
        
        if cave[y*WIDTH + x] != AIR {
            if cave[y*WIDTH + x - 1] != AIR {
                if cave[y*WIDTH + x + 1] != AIR {
                    cave[(y-1)*WIDTH + x] = SAND;
                    return true
                } else {
                    x += 1;
                }
            } else {
                x -= 1;
            }
        }
    }
}

fn add_sand_until_it_falls_off(cave: &mut [u8; WIDTH*HEIGHT]) -> usize {
    let mut p1 = 0;
    while introduce_sand(cave, START_X, START_Y) {
        p1 += 1;
    }
    p1
}

fn add_sand_until_full(cave: &mut [u8; WIDTH*HEIGHT]) -> usize {
    let mut p2 = 0;
    while cave[START_Y*WIDTH + START_X] == AIR && introduce_sand(cave, START_X, START_Y) {
        p2 += 1;
    }
    p2
}

pub fn solve(input: &str) -> Solution {
    let mut cave: Cave = [AIR; WIDTH*HEIGHT];

    let mut bottom = 0;
    for line in input.lines() {
        let groups: Vec<(usize,usize)> = line.split(" -> ").filter_map(|g| g
                .split_once(",")
                .map(|(s1,s2)| {
                    let n1 = s1.parse::<usize>().unwrap();
                    let n2 = s2.parse::<usize>().unwrap();
                    assert!(n1 >= X_OFFSET, "Column too far to the left");
                    assert!(n1 < X_OFFSET + WIDTH, "Too narrow");
                    assert!(n2 < HEIGHT, "Too low");
                    (n1 - X_OFFSET, n2)
                }))    
                .collect();

        bottom = bottom.max(groups.iter().map(|g| g.1).max().unwrap());

        for g in groups.windows(2) {
            for row in g[0].1.min(g[1].1)..=g[0].1.max(g[1].1) {
                for column in g[0].0.min(g[1].0)..=g[0].0.max(g[1].0) {
                    cave[row*WIDTH + column] = ROCK;
                }
            }
        }
    }

    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         print!("{}",
    //         match cave[y*WIDTH + x] {
    //             AIR => '.',
    //             SAND => 'o',
    //             ROCK => '#',
    //             _ => '?',
    //         });
    //     }
    //     println!("");
    // }

    let p1 = add_sand_until_it_falls_off(&mut cave);

    for x in 0..WIDTH {
        cave[(bottom + 2)*WIDTH + x] = ROCK;
    }

    let p2 = add_sand_until_full(&mut cave) + p1;

    Solution::new(p1,p2)
}
