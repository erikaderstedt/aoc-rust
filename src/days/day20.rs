use crate::common::Solution;
use std::collections::HashMap;
use pathfinding::prelude::bfs;
use std::hash::{Hash, Hasher};

type Tile = [[bool;10];10];
type Image = Vec<Vec<bool>>;

enum Edge { Top, Bottom,Left,Right }

const HASH: u8 = '#' as u8;
const TOP: usize = 0;
const BOTTOM: usize = 9;
const LEFT: usize = 0;
const RIGHT: usize = 9;

fn get_edge(tile: &Tile, edge: Edge) -> u16 {
    // left to right, top to bottom
    match edge {
        Edge::Top => tile[TOP].iter().fold(0, |acc,v| (acc << 1) + if *v { 1 } else { 0 }),
        Edge::Bottom => tile[BOTTOM].iter().fold(0, |acc,v| (acc << 1) + if *v { 1 } else { 0 }),
        Edge::Left => (TOP..=BOTTOM).fold(0, |acc,v| (acc << 1) + if tile[v][LEFT] { 1 } else { 0 }),
        Edge::Right => (TOP..=BOTTOM).fold(0, |acc,v| (acc << 1) + if tile[v][RIGHT] { 1 } else { 0 })
    }
}

fn flip_tile(tile: &Tile) -> Tile {
    [tile[9],tile[8],tile[7],tile[6],tile[5],tile[4],tile[3],tile[2],tile[1],tile[0],]
}

fn rotate_tile_right(t: &Tile) -> Tile {
    [[t[9][0], t[8][0], t[7][0], t[6][0], t[5][0], t[4][0], t[3][0], t[2][0], t[1][0], t[0][0],],
    [t[9][1], t[8][1], t[7][1], t[6][1], t[5][1], t[4][1], t[3][1], t[2][1], t[1][1], t[0][1],],
    [t[9][2], t[8][2], t[7][2], t[6][2], t[5][2], t[4][2], t[3][2], t[2][2], t[1][2], t[0][2],],
    [t[9][3], t[8][3], t[7][3], t[6][3], t[5][3], t[4][3], t[3][3], t[2][3], t[1][3], t[0][3],],
    [t[9][4], t[8][4], t[7][4], t[6][4], t[5][4], t[4][4], t[3][4], t[2][4], t[1][4], t[0][4],],
    [t[9][5], t[8][5], t[7][5], t[6][5], t[5][5], t[4][5], t[3][5], t[2][5], t[1][5], t[0][5],],
    [t[9][6], t[8][6], t[7][6], t[6][6], t[5][6], t[4][6], t[3][6], t[2][6], t[1][6], t[0][6],],
    [t[9][7], t[8][7], t[7][7], t[6][7], t[5][7], t[4][7], t[3][7], t[2][7], t[1][7], t[0][7],],
    [t[9][8], t[8][8], t[7][8], t[6][8], t[5][8], t[4][8], t[3][8], t[2][8], t[1][8], t[0][8],],
    [t[9][9], t[8][9], t[7][9], t[6][9], t[5][9], t[4][9], t[3][9], t[2][9], t[1][9], t[0][9],],]
}

#[allow(dead_code)]
fn show_tile(t: &Tile, name: &str) {
    println!("\nTile {}", name);
    for row in 0..10 {
        let s: String = t[row].iter().map(|&v| if v { '#' } else { '.' }).collect();
        println!("{}",s)
    }
}

fn variants(t: &Tile) -> [Tile;8] {
    let r1 = rotate_tile_right(t);
    let r2 = rotate_tile_right(&r1);
    let r3 = rotate_tile_right(&r2);
    let f1 = flip_tile(t);
    let f2 = flip_tile(&r2);
    let f3 = flip_tile(&r3);
    let f4 = flip_tile(&r1);
    [t.clone(), f1,f2,f3,f4,r1,r2,r3]
}

fn tile_fits(t: &Tile, above: Option<u16>, left: Option<u16>) -> bool {
    (!matches!(above, Some(e) if e != get_edge(t, Edge::Top))) &&
    (!matches!(left, Some(e) if e != get_edge(t, Edge::Left)))
}

const TILE_SIDE: usize = 10;

#[derive(Clone,PartialEq,Eq,Debug)]
struct PlacedTile {
    index: usize,
    right: u16,
    bottom: u16,
    tile: Tile,
}

impl Hash for PlacedTile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PlacedTile {
    fn from_tile(index: &usize, tile: &Tile) -> PlacedTile {
        PlacedTile { index: index.clone(), 
            right: get_edge(tile, Edge::Right), 
            bottom: get_edge(tile, Edge::Bottom),
            tile: tile.clone(),
        }
    }
}

#[derive(Clone,PartialEq,Eq)]
struct State {
    remaining_tiles: HashMap<usize,Tile>,
    side: usize,
    grid: Vec<Option<PlacedTile>>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl State {

    fn from_tiles(tiles: HashMap<usize,Tile>) -> State {
        let side = match tiles.len() {
            144 => 12,
            9 => 3,
            _ => panic!("Unknown size"),
        };
        let grid = vec![None; tiles.len()];
        State { remaining_tiles: tiles, side, grid, }
    }

    fn all_laid_out(&self) -> bool {
        self.grid.iter().all(|u| u.is_some())
    }

    fn corner_tiles_product(&self) -> usize {
        self.grid[0].as_ref().unwrap().index *
        self.grid[self.side - 1].as_ref().unwrap().index *
        self.grid[self.grid.len()-1].as_ref().unwrap().index *
        self.grid[self.grid.len()-self.side].as_ref().unwrap().index
    }

    fn successors(&self) -> Vec<State> {
        match self.grid.iter().enumerate().find(|(i,g)| g.is_none()) {
            Some((i,_)) => {
                // println!("{}", i);
                // Get edge values for last above and last to the left
                let above: Option<u16> = if i >= self.side { Some(self.grid[i-self.side].as_ref().unwrap().bottom) } else { None };
                let left: Option<u16> = if (i % self.side) > 0 { Some(self.grid[i-1].as_ref().unwrap().right) } else { None };

                self.remaining_tiles.iter()
                    .map(|(r, t)| -> Vec<PlacedTile> {
                        variants(t).iter()
                            .filter(|tile_variant| tile_fits(tile_variant, above,left))
                            .map(|tile_variant: &Tile| PlacedTile::from_tile(r, tile_variant))
                            .collect()
                    })
                    .flatten()
                    .map(|pt: PlacedTile| -> State {
                        let mut c = self.remaining_tiles.clone();
                        c.remove(&pt.index);
                        let mut g = self.grid.clone();
                        g[i] = Some(pt);
                        State { remaining_tiles: c, side: self.side, grid: g }
                    })
                    .collect()
                

            },
            None => { vec![] },
        }
    }

    fn get_image(&self) -> Image {
        let mut v: Image = Vec::new();
        let mut i = 0;
        for row in 0..self.side {
            for row_in_tile in 1..=(TILE_SIDE - 2) {
                v.push(Vec::new());
                for column in 0..self.side {
                    let placed: &PlacedTile = self.grid[column+row*self.side].as_ref().unwrap();
                    v[i].extend(placed.tile[row_in_tile].iter().skip(1).take(TILE_SIDE-2))
                }
                i += 1;
            }
        }
        v
    }

}

fn rotate_grid(g: &Image) -> Image {
    let s = g.len();
    (0..s).map(|r| 
        (0..s).map(|c| g[c][r]).collect()
    ).collect()
}

fn flip_grid(g: &Image) -> Image {
    g.iter().rev().cloned().collect()
}

const SEA_MONSTER_PIXELS: usize = 15;
const SEA_MONSTER_WIDTH: usize = 20;
const SEA_MONSTER_HEIGHT: usize = 3;

fn num_sea_monsters_in_image(image: &Image) -> usize {
    let s = image.len();
    (0..(s - SEA_MONSTER_HEIGHT)).map(|r| 
        (0..(s-SEA_MONSTER_WIDTH)).filter(|c| sea_monster_at(&r, c, image)).count()
    ).sum::<usize>()
}

fn sea_monster_at(row: &usize, col: &usize, grid: &Image) -> bool {
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

fn sea_roughness(grid:&Image, num_sea_monsters: &usize) -> usize {
    grid.iter().map(|g| g.iter().filter(|&x| *x).count()).sum::<usize>() - *num_sea_monsters*SEA_MONSTER_PIXELS
}

pub fn solve(input: &str) -> Solution {
    let lines: Vec<&str> = input.lines().collect();
    let mut tiles: HashMap<usize,Tile> = HashMap::new();
    for line_index in (0..lines.len()).step_by(TILE_SIDE + 2) {
        let c = lines[line_index].split(' ').skip(1).next().unwrap().split(':').next().unwrap().parse::<usize>().unwrap();
        let tile:Vec<[bool;10]> = (0..TILE_SIDE).map(|i| {
            let b0 = lines[line_index+i+1].as_bytes();
            [b0[0] == HASH, b0[1] == HASH,b0[2] == HASH, b0[3] == HASH,b0[4] == HASH, b0[5] == HASH,b0[6] == HASH, b0[7] == HASH,b0[8] == HASH, b0[9] == HASH,]
        }).collect();

        tiles.insert(c, [tile[0],tile[1],tile[2],tile[3],tile[4],tile[5],tile[6],tile[7],tile[8],tile[9],]);
    }

    let result = bfs(&State::from_tiles(tiles), State::successors, State::all_laid_out).expect("No solution pt 1");
    let final_state = &result[result.len() - 1];

    let p1 = final_state.corner_tiles_product();
    println!("p1 {}", p1);
    let image = final_state.get_image();
//    println!("{:?}", image);
    println!("got image {} {}", image.len(), image[0].len());
    let i1 = rotate_grid(&image);
    let i2 = rotate_grid(&i1);
    let i3 = rotate_grid(&i2);
    let f1 = flip_grid(&image);
    let f2 = flip_grid(&i1);
    let f3 = flip_grid(&i2);
    let f4 = flip_grid(&i3);
    let monsters: Vec<(usize,usize)> = 
    [image, i1, i2, i3, f1, f2, f3, f4].into_iter().map(|image| -> (usize, usize) {
        let n = num_sea_monsters_in_image(&image);
        let r = sea_roughness(&image, &n);
        (n, r)
     }).collect();

    println!("{:?}", monsters);
    

    
    Solution { part_1: p1.to_string(), part_2: "".to_string() }
}