use crate::common::Solution;
use pathfinding::prelude::bfs;
use std::hash::{Hash, Hasher};
use std::fmt;

type Pixels = Vec<Vec<bool>>;
const TILE_SIDE: usize = 10;
const HASH: u8 = '#' as u8;

enum Edge { Top, Bottom,Left,Right }

#[derive(Clone,PartialEq,Eq)]
struct Tile {
    index: usize,
    variant: usize,
    data: [[bool;10];10],
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.variant.hash(state)
    }
}

impl Tile {

    fn from_str(ch: &str) -> Tile {
        let tile:Vec<[bool;10]> = ch.lines().skip(1).take(TILE_SIDE).map(|s| {
            let b0 = s.as_bytes();
            [b0[0] == HASH, b0[1] == HASH,b0[2] == HASH, b0[3] == HASH,b0[4] == HASH, b0[5] == HASH,b0[6] == HASH, b0[7] == HASH,b0[8] == HASH, b0[9] == HASH,]
        }).collect();

        Tile { 
            index: ch[..4].parse::<usize>().unwrap(),
            variant: 0,
            data: [tile[0],tile[1],tile[2],tile[3],tile[4],tile[5],tile[6],tile[7],tile[8],tile[9],]
        }
    }

    fn flip(&self) -> Tile {
        Tile { index: self.index,
        variant: self.variant + 4,
        data: [self.data[9],self.data[8],self.data[7],self.data[6],self.data[5],self.data[4],self.data[3],self.data[2],self.data[1],self.data[0],] }
    }
    
    fn rotate_right(&self) -> Tile {
        Tile { index: self.index, variant: self.variant + 1, data: 
        [[self.data[9][0], self.data[8][0], self.data[7][0], self.data[6][0], self.data[5][0], self.data[4][0], self.data[3][0], self.data[2][0], self.data[1][0], self.data[0][0],],
        [self.data[9][1], self.data[8][1], self.data[7][1], self.data[6][1], self.data[5][1], self.data[4][1], self.data[3][1], self.data[2][1], self.data[1][1], self.data[0][1],],
        [self.data[9][2], self.data[8][2], self.data[7][2], self.data[6][2], self.data[5][2], self.data[4][2], self.data[3][2], self.data[2][2], self.data[1][2], self.data[0][2],],
        [self.data[9][3], self.data[8][3], self.data[7][3], self.data[6][3], self.data[5][3], self.data[4][3], self.data[3][3], self.data[2][3], self.data[1][3], self.data[0][3],],
        [self.data[9][4], self.data[8][4], self.data[7][4], self.data[6][4], self.data[5][4], self.data[4][4], self.data[3][4], self.data[2][4], self.data[1][4], self.data[0][4],],
        [self.data[9][5], self.data[8][5], self.data[7][5], self.data[6][5], self.data[5][5], self.data[4][5], self.data[3][5], self.data[2][5], self.data[1][5], self.data[0][5],],
        [self.data[9][6], self.data[8][6], self.data[7][6], self.data[6][6], self.data[5][6], self.data[4][6], self.data[3][6], self.data[2][6], self.data[1][6], self.data[0][6],],
        [self.data[9][7], self.data[8][7], self.data[7][7], self.data[6][7], self.data[5][7], self.data[4][7], self.data[3][7], self.data[2][7], self.data[1][7], self.data[0][7],],
        [self.data[9][8], self.data[8][8], self.data[7][8], self.data[6][8], self.data[5][8], self.data[4][8], self.data[3][8], self.data[2][8], self.data[1][8], self.data[0][8],],
        [self.data[9][9], self.data[8][9], self.data[7][9], self.data[6][9], self.data[5][9], self.data[4][9], self.data[3][9], self.data[2][9], self.data[1][9], self.data[0][9],],]}
    }

    fn variants(&self) -> [Tile;8] {
        let t1 = self.rotate_right(); let t2 = t1.rotate_right(); let t3 = t2.rotate_right();
        let f1 = self.flip(); let f2 = t1.flip(); let f3 = t2.flip(); let f4 = t3.flip();
        [self.clone(), t1,t2,t3,f1,f2,f3,f4]
    }

    fn edge_value(&self, edge: Edge) -> u16 {
        match edge {
            Edge::Top => self.data[0].iter().fold(0, |acc,v| (acc << 1) + if *v { 1 } else { 0 }),
            Edge::Bottom => self.data[TILE_SIDE-1].iter().fold(0, |acc,v| (acc << 1) + if *v { 1 } else { 0 }),
            Edge::Left => (0..TILE_SIDE).fold(0, |acc,v| (acc << 1) + if self.data[v][0] { 1 } else { 0 }),
            Edge::Right => (0..TILE_SIDE).fold(0, |acc,v| (acc << 1) + if self.data[v][TILE_SIDE-1] { 1 } else { 0 })
        }
    }

    fn fits(&self, above: Option<u16>, left: Option<u16>) -> bool {
        (!matches!(above, Some(e) if e != self.edge_value(Edge::Top))) &&
        (!matches!(left, Some(e) if e != self.edge_value(Edge::Left)))
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {} - variant {}", self.index, self.variant)?;
        for r in 0..TILE_SIDE {
            let s: String = self.data[r].iter().map(|v| if *v { '#' } else { '.' }).collect();
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[derive(Clone,PartialEq,Eq)]
struct State {
    remaining_tiles: Vec<Tile>,
    side: usize,
    grid: Vec<Tile>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl State {

    fn from_tiles(tiles: Vec<Tile>) -> State {
        let side = match tiles.len() {
            144 => 12,
            9 => 3,
            _ => panic!("Unknown size"),
        };
        State { remaining_tiles: tiles, side, grid: Vec::new(), }
    }

    fn all_laid_out(&self) -> bool { self.grid.len() == self.side * self.side }

    fn corner_tiles_product(&self) -> usize {
        self.grid[0].index *
        self.grid[self.side - 1].index *
        self.grid[self.grid.len()-1].index *
        self.grid[self.grid.len()-self.side].index
    }

    fn successors(&self) -> Vec<State> {
        let i = self.grid.len();
        let above: Option<u16> = if i >= self.side { Some(self.grid[i-self.side].edge_value(Edge::Bottom)) } else { None };
        let left: Option<u16> = if (i % self.side) > 0 { Some(self.grid[i-1].edge_value(Edge::Right)) } else { None };

        self.remaining_tiles.iter()
            .map(|t| -> Vec<Tile> {
                t.variants().iter().filter(|tile_variant| tile_variant.fits(above,left)).cloned().collect()
            })
            .flatten()
            .map(|pt: Tile| -> State {
                let mut c = self.remaining_tiles.clone();
                let j = c.iter().position(|x| x.index == pt.index).unwrap();
                c.remove(j);
                let mut g = self.grid.clone();
                g.push(pt);
                State { remaining_tiles: c, side: self.side, grid: g }
            })
            .collect()
    }

    fn get_image(&self) -> Pixels {
        let mut v: Pixels = Vec::new();
        let mut i = 0;
        for row in 0..self.side {
            for row_in_tile in 1..=(TILE_SIDE - 2) {
                v.push(Vec::new());
                for column in 0..self.side {
                    v[i].extend(self.grid[column+row*self.side].data[row_in_tile].iter().skip(1).take(TILE_SIDE-2))
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
    (0..(s - SEA_MONSTER_HEIGHT)).map(|r| 
        (0..(s-SEA_MONSTER_WIDTH)).filter(|c| sea_monster_at(&r, c, image)).count()
    ).sum::<usize>()
}

fn sea_monster_at(row: &usize, col: &usize, grid: &Pixels) -> bool {
    row + 2 < grid.len() &&
    col + 19 < grid[0].len() &&
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
    let tiles: Vec<Tile> = input.split("Tile ").skip(1).map(|s| Tile::from_str(s)).collect();
    println!("{}", tiles.len());

    let result = bfs(&State::from_tiles(tiles), State::successors, State::all_laid_out).expect("No solution pt 1");
    let final_state = &result[result.len() - 1];

    let p1 = final_state.corner_tiles_product();

    let p2 = variants(&final_state.get_image()).iter().map(|image| -> (usize, usize) {
        let n = num_sea_monsters_in_image(&image);
        let r = sea_roughness(&image, &n);
        (n, r)
     }).max_by(|a,b| a.0.cmp(&b.0)).unwrap().1;
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}