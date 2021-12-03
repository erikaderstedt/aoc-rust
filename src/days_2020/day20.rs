use crate::common::Solution;
use pathfinding::prelude::dfs;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;

const TILE_SIDE: usize = 10;
type TileData = [[bool;TILE_SIDE];TILE_SIDE];
type Pixels = Vec<Vec<bool>>;
type Tile = u64;
type Index = u64;
const LEFT_OFFSET: u64 = 16;
const RIGHT_OFFSET: u64 = 28;
const TOP_OFFSET: u64 = 40;
const BOTTOM_OFFSET: u64 = 52;

fn index(tile: &u64) -> Index { tile & 4095 }
fn variant(tile: &u64) -> u64 { (tile >> 12) & 7 }
fn left_edge(tile: &u64) -> u64 { (tile >> LEFT_OFFSET) & 1023 }
fn right_edge(tile: &u64) -> u64 { (tile >> RIGHT_OFFSET) & 1023 }
fn top_edge(tile: &u64) -> u64 { (tile >> TOP_OFFSET) & 1023 }
fn bottom_edge(tile: &u64) -> u64 { (tile >> BOTTOM_OFFSET) & 1023 }

fn reverse(n: u64) -> u64 {
    assert!(n < 1024);
    ((n & 1) << 9) + ((n & 2) << 7) + ((n & 4) << 5) + ((n & 8) << 3) + ((n & 16) << 1) +
    (n >> 9) + ((n >> 7) & 2) + ((n >> 5) & 4) + ((n >> 3) & 8) + ((n >> 1) & 16)
}

fn build_tile(index: &u64, variant: u64, top: u64, right: u64, bottom: u64, left: u64, ) -> Tile {
    index + (variant << 12) + (left << LEFT_OFFSET) + (right << RIGHT_OFFSET) + (top << TOP_OFFSET) + (bottom << BOTTOM_OFFSET)
}

fn create_tile_from_tiledata(index: &u64, data: &TileData) -> Tile {
    // Left to right, top to bottom. High bits first
    let top = data[0].iter().fold(0u64, |acc, b| (acc << 1) + if *b { 1 } else { 0 });
    let bottom = data[TILE_SIDE-1].iter().fold(0u64, |acc, b| (acc << 1) + if *b { 1 } else { 0 });
    let left = data.iter().fold(0u64, |acc, b| (acc << 1) + if b[0] { 1 } else { 0 });
    let right = data.iter().fold(0u64, |acc, b| (acc << 1) + if b[TILE_SIDE-1] { 1 } else { 0 });

    build_tile(index, 0, top, right, bottom, left)
}

fn is_starting_piece(tile: &u64, tiles: &Vec<u64>) -> bool {
    let t = top_edge(tile);  let l = left_edge(tile);
    let any_match_for_top_edge = tiles.iter().filter(|other| index(other) != index(tile)).any(|other| t == bottom_edge(other));
    let any_match_for_left_edge = tiles.iter().filter(|other| index(other) != index(tile)).any(|other| l == right_edge(other));
    !any_match_for_left_edge && !any_match_for_top_edge
}

fn tile_variants(tile: &u64) -> [Tile;8] {
    let index = index(tile);
    let l = left_edge(tile);    let rev_l = reverse(l); 
    let r = right_edge(tile);   let rev_r = reverse(r);
    let b = bottom_edge(tile);  let rev_b = reverse(b);
    let t = top_edge(tile);     let rev_t = reverse(t);
    [tile.clone(),          //TOP       RIGHT   BOTTOM  LEFT
    //  build_tile(&index, 0, t,        r,      b,      l),
        build_tile(&index, 1, rev_l,    t,      rev_r,  b), 
        build_tile(&index, 2, rev_b,    rev_l,  rev_t,  rev_r),
        build_tile(&index, 3, r,        rev_b,  l,      rev_t),
        build_tile(&index, 4, b,        rev_r,  t,      rev_l),
        build_tile(&index, 5, rev_r,    rev_t,  rev_l,  rev_b),
        build_tile(&index, 6, rev_t,    l,      rev_b,  r),
        build_tile(&index, 7, l,        b,      r,      t)]
}

fn tile_fits(tile: &u64, above: Option<u64>, left: Option<u64>) -> bool {
    (!matches!(above, Some(bottom) if bottom != top_edge(tile))) &&
    (!matches!(left, Some(right) if right != left_edge(tile)))
}

fn flip(data: &TileData) -> TileData {
    [data[9],data[8],data[7],data[6],data[5],data[4],data[3],data[2],data[1],data[0],]
}

fn rotate_right(data: &TileData) -> TileData {
    [[data[9][0], data[8][0], data[7][0], data[6][0], data[5][0], data[4][0], data[3][0], data[2][0], data[1][0], data[0][0],],
    [data[9][1], data[8][1], data[7][1], data[6][1], data[5][1], data[4][1], data[3][1], data[2][1], data[1][1], data[0][1],],
    [data[9][2], data[8][2], data[7][2], data[6][2], data[5][2], data[4][2], data[3][2], data[2][2], data[1][2], data[0][2],],
    [data[9][3], data[8][3], data[7][3], data[6][3], data[5][3], data[4][3], data[3][3], data[2][3], data[1][3], data[0][3],],
    [data[9][4], data[8][4], data[7][4], data[6][4], data[5][4], data[4][4], data[3][4], data[2][4], data[1][4], data[0][4],],
    [data[9][5], data[8][5], data[7][5], data[6][5], data[5][5], data[4][5], data[3][5], data[2][5], data[1][5], data[0][5],],
    [data[9][6], data[8][6], data[7][6], data[6][6], data[5][6], data[4][6], data[3][6], data[2][6], data[1][6], data[0][6],],
    [data[9][7], data[8][7], data[7][7], data[6][7], data[5][7], data[4][7], data[3][7], data[2][7], data[1][7], data[0][7],],
    [data[9][8], data[8][8], data[7][8], data[6][8], data[5][8], data[4][8], data[3][8], data[2][8], data[1][8], data[0][8],],
    [data[9][9], data[8][9], data[7][9], data[6][9], data[5][9], data[4][9], data[3][9], data[2][9], data[1][9], data[0][9],],]
}

const HASH: u8 = '#' as u8;

#[derive(Clone,Eq)]
struct State {
    remaining_tiles: Vec<u64>,
    side: usize,
    grid: Vec<u64>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl State {

    fn from_tiles(tiles: Vec<Tile>) -> State {
        let side = match tiles.len() {
            1152 => 12,
            72 => 3,
            _ => panic!("Unknown size"),
        };

        let start = tiles.iter().find(|t| is_starting_piece(t, &tiles)).unwrap().clone();
        let mut grid = Vec::with_capacity(tiles.len());
        let tiles = tiles.into_iter().filter(|t| index(t) != index(&start) ).collect();
        grid.push(start);
        
        State { remaining_tiles: tiles, side, grid, }
    }

    fn all_laid_out(&self) -> bool { self.grid.len() == self.side * self.side }

    fn corner_tiles_product(&self) -> u64 {
        index(&self.grid[0]) *
        index(&self.grid[self.side - 1]) *
        index(&self.grid[self.grid.len()-1]) *
        index(&self.grid[self.grid.len()-self.side])
    }

    fn successors(&self) -> Vec<State> {
        let i = self.grid.len();
        let above: Option<u64> = if i >= self.side { Some(bottom_edge(&self.grid[i-self.side])) } else { None };
        let left: Option<u64> = if (i % self.side) > 0 { Some(right_edge(&self.grid[i-1])) } else { None };
        self.remaining_tiles.iter()
            .filter(|tile| tile_fits(tile, above,left))
            .map(|pt: &Tile| -> State {
                let tile_index = index(pt);
                let c:Vec<Tile> = self.remaining_tiles.iter().filter(|x| index(x) != tile_index).cloned().collect();
                let mut g = self.grid.clone();
                g.push(pt.clone());
                State { remaining_tiles: c, side: self.side, grid: g }
            })
            .collect()
    }

    fn get_image(&self, raw_tiles: &HashMap<Index,TileData>) -> Pixels {
        let grid: Vec<TileData> = self.grid.iter().map(|tile| {
            let raw_tile = raw_tiles[&index(tile)];
            let rotated = match variant(tile) % 4 {
                0 => raw_tile,
                1 => rotate_right(&raw_tile),
                2 => rotate_right(&rotate_right(&raw_tile)),
                3 => rotate_right(&rotate_right(&rotate_right(&raw_tile))),
                _ => unreachable!(),
            };
            if variant(tile) >= 4 { flip(&rotated) } else { rotated }
        }).collect();
        let mut v: Pixels = Vec::new();
        let mut i = 0;
        for row in 0..self.side {
            for row_in_tile in 1..=(TILE_SIDE - 2) {
                v.push(Vec::new());
                for column in 0..self.side {
                    v[i].extend(grid[column+row*self.side][row_in_tile].iter().skip(1).take(TILE_SIDE-2))
                }
                i += 1;
            }
        }
        v
    }
}

fn rotate_pixels(g: &Pixels) -> Pixels { (0..g.len()).map(|r| (0..g.len()).rev().map(|c| g[c][r]).collect()).collect() }
fn flip_pixels(g: &Pixels) -> Pixels { g.iter().rev().cloned().collect() }
fn variants(g: &Pixels) -> [Pixels;8] {
    let r1 = rotate_pixels(g); let r2 = rotate_pixels(&r1); let r3 = rotate_pixels(&r2);
    let f1 = flip_pixels(g); let f2 = flip_pixels(&r1); let f3 = flip_pixels(&r2); let f4 = flip_pixels(&r3);
    [g.clone(),r1,r2,r3,f1,f2,f3,f4]
}

const SEA_MONSTER_PIXELS: usize = 15;
const SEA_MONSTER_WIDTH: usize = 20;
const SEA_MONSTER_HEIGHT: usize = 3;

fn num_sea_monsters_in_image(image: &Pixels) -> usize {
    let s = image.len();
    (0..(s-SEA_MONSTER_HEIGHT)).map(|r| 
        (0..(s-SEA_MONSTER_WIDTH)).filter(|c| sea_monster_at(r, c.clone(), image)).count()
    ).sum::<usize>()
}

fn sea_monster_at(row: usize, col: usize, grid: &Pixels) -> bool {
    // 01234567890123456789
    //                   # 
    // #    ##    ##    ###
    //  #  #  #  #  #  #   
    grid[row+1][col+0] &&
    grid[row+2][col+1] &&
    grid[row+2][col+4] &&
    grid[row+1][col+5] &&
    grid[row+1][col+6] &&
    grid[row+2][col+7] &&
    grid[row+2][col+10] &&
    grid[row+1][col+11] &&
    grid[row+1][col+12] &&
    grid[row+2][col+13] &&
    grid[row+2][col+16] &&
    grid[row+1][col+17] &&
    grid[row+0][col+18] &&
    grid[row+1][col+18] &&
    grid[row+1][col+19]
}

fn sea_roughness(grid:&Pixels, num_sea_monsters: &usize) -> usize {
    grid.iter().map(|g| g.iter().filter(|&x| *x).count()).sum::<usize>() - *num_sea_monsters*SEA_MONSTER_PIXELS
}

pub fn solve(input: &str) -> Solution {
    let tiles: HashMap<Index,TileData> = input.split("Tile ").skip(1).map(|s| {
        let row:Vec<[bool;10]> = s.lines().skip(1).take(TILE_SIDE).map(|s| {
            let b0 = s.as_bytes();
            [b0[0] == HASH, b0[1] == HASH,b0[2] == HASH, b0[3] == HASH,b0[4] == HASH, b0[5] == HASH,b0[6] == HASH, b0[7] == HASH,b0[8] == HASH, b0[9] == HASH,]
        }).collect();
        let num = s[..4].parse::<u64>().unwrap();
        (num,[row[0],row[1],row[2],row[3],row[4],row[5],row[6],row[7],row[8],row[9],])
    }).collect();

    let compressed_tiles: Vec<Tile> = tiles.iter().map(|(index,data)| {
        let tile = create_tile_from_tiledata(index, data);
        tile_variants(&tile).to_vec()
    }).flatten().collect();
    
    let result = dfs(State::from_tiles(compressed_tiles), State::successors, State::all_laid_out).expect("No solution pt 1");
    let final_state = &result[result.len() - 1];

    let p1 = final_state.corner_tiles_product();

    let p2 = variants(&final_state.get_image(&tiles)).iter().map(|image| -> (usize, usize) {
        let n = num_sea_monsters_in_image(&image);
        let r = sea_roughness(&image, &n);
        (n, r)
     }).max_by(|a,b| a.0.cmp(&b.0)).unwrap().1;
    
    Solution::new(p1,p2)
}
