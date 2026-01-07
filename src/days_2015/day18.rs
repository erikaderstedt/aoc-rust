// https://adventofcode.com/2015/day/18

use crate::{
    common::Solution,
    grid::{Grid, GridElement},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Light {
    On,
    Off,
}

impl GridElement for Light {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Self::On),
            '.' => Some(Self::Off),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::Off => '.',
            Self::On => '#',
        }
    }
}

fn flip_lights<const STUCK: bool>(mut grid: Grid<Light>) -> usize {
    if STUCK {
        grid.locations[0] = Light::On;
        grid.locations[grid.cols - 1] = Light::On;
        grid.locations[(grid.cols - 1) * grid.rows] = Light::On;
        grid.locations[grid.cols * grid.rows - 1] = Light::On;
    }
    for _ in 0..100 {
        let mut new_grid = Grid {
            rows: grid.rows,
            cols: grid.cols,
            locations: vec![Light::Off; grid.locations.len()],
        };
        for p in grid.positions() {
            let n = match grid.get(&p).unwrap() {
                Light::On => {
                    let on_neighbors = p
                        .neighbors()
                        .filter(|q| grid.get(q) == Some(Light::On))
                        .count();
                    if on_neighbors < 2 || on_neighbors > 3 {
                        Light::Off
                    } else {
                        Light::On
                    }
                }
                Light::Off => {
                    if p.neighbors()
                        .filter(|q| grid.get(q) == Some(Light::On))
                        .count()
                        == 3
                    {
                        Light::On
                    } else {
                        Light::Off
                    }
                }
            };
            new_grid.set(&p, n);
        }
        if STUCK {
            new_grid.locations[0] = Light::On;
            new_grid.locations[new_grid.cols - 1] = Light::On;
            new_grid.locations[(new_grid.cols - 1) * new_grid.rows] = Light::On;
            new_grid.locations[new_grid.cols * new_grid.rows - 1] = Light::On;
        }
        grid = new_grid;
    }
    grid.locations
        .into_iter()
        .filter(|&v| v == Light::On)
        .count()
}

pub fn solve(input: &str) -> Solution {
    let grid: Grid<Light> = Grid::load(input);

    let p1 = flip_lights::<false>(grid.clone());
    let p2 = flip_lights::<true>(grid.clone());

    Solution::new(p1, p2)
}
