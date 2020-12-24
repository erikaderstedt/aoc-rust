use crate::common::Solution;
use std::collections::HashSet;

const W: u8 = 'w' as u8;
const E: u8 = 'e' as u8;
const N: u8 = 'n' as u8;
const S: u8 = 's' as u8;

fn neighbors(x: i32,y: i32) -> [(i32,i32); 6] {
    if (y % 2) != 0 {
        [(x, y+1), (x+1,y+1), (x, y-1), (x+1,y-1), (x-1,y), (x+1,y)]
    } else {
        [(x-1, y+1), (x,y+1), (x-1, y-1), (x,y-1), (x-1,y), (x+1,y)]
    }
}

pub fn solve(input: &str) -> Solution {
    let mut black_tiles: HashSet<(i32,i32)> = HashSet::new();

    for line in input.lines() {
        let (x,y) = line.as_bytes().iter().scan(((0i32,0i32), false), |state, b| {
            let mut x = state.0.0; let mut y = state.0.1;
            if state.1 {
                match *b {
                    E => { if (y % 2) == 0 { x += 1; } },
                    W => { if (y % 2) != 0 { x -= 1; } },
                    _ => unreachable!(),
                }
            } else {
                match *b {
                    W => x -= 1,
                    E => x += 1,
                    N => y += 1,
                    S => y -= 1,
                    _ => unreachable!(),
                }
            }
            *state = ((x,y), matches!(*b, N | S));
            Some((x,y))
        }).last().unwrap();
      
        if !black_tiles.insert((x,y)) { black_tiles.remove(&(x,y)); }
    }
    let p1 = black_tiles.len();

    for _day in 0..100 {
        let mut n: HashSet<(i32,i32)> = HashSet::new();
        for (x,y) in black_tiles.iter().cloned() {
            n.extend(&neighbors(x,y));
            n.insert((x,y));
        }
        black_tiles = n.into_iter().filter(|(x,y)| {
            let x = x.clone(); let y = y.clone();
            let num_black_neighbors = neighbors(x,y).iter().filter(|p| black_tiles.contains(p)).count();
            let is_black = black_tiles.contains(&(x,y));

            if is_black && (num_black_neighbors == 0 || num_black_neighbors > 2) {
                return false;
            } else if !is_black && num_black_neighbors == 2 {
                return true;
            } else {
                return is_black;
            }
        }).collect();
    }
    let p2 = black_tiles.len();

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}
