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

const TILES_WIDTH: usize = 256;
const TILES_HALF_WIDTH: usize = 128;

struct Tiles {
    tiles: [bool;TILES_WIDTH*TILES_WIDTH]
}

impl Tiles {
    fn new() -> Tiles { Tiles { tiles: [false; 256*256]}}
    fn flip(&self, x: i32, y: i32) {
        let i = ((y+TILES_HALF_WIDTH) as usize) * TILES_WIDTH + (x + TILES_HALF_WIDTH) as usize;
        self.tiles[i] = !self.tiles[i];
    }

    fn is_black(&self, x: i32, y: i32) -> bool {
        let i = ((y+TILES_HALF_WIDTH) as usize) * TILES_WIDTH + (x + TILES_HALF_WIDTH) as usize;
        self.tiles[i]
    }

    fn num_black_tiles(&self) -> usize {
        self.tiles.iter().filter(|&v| *v).count()
    }

    fn neighbors(&self, x: i32, y: i32) {
        
    }

}

pub fn solve(input: &str) -> Solution {
    let mut tiles = Tiles::new();

    for line in input.lines() {
        let (x,y) = line.as_bytes().iter().scan(((0i32,0i32), false), |state, b| {
            let mut x = state.0.0; let mut y = state.0.1;
            match (state.1, *b) {
                (true, E) => { if (y % 2) == 0 { x += 1; } },
                (true, W) => { if (y % 2) != 0 { x -= 1; } },
                (false,W) => x -= 1,
                (false,E) => x += 1,
                (false,N) => y += 1,
                (false,S) => y -= 1,
                _ => unreachable!(),
            }
            *state = ((x,y), matches!(*b, N | S));
            Some((x,y))
        }).last().unwrap();
        
        tiles.flip(x,y);
    }
    let p1 = tiles.num_black_tiles();

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
