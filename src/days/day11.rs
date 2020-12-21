use crate::common::Solution;

const OCCUPIED: u8 = '#' as u8;
const EMPTY: u8 = 'L' as u8;
const FLOOR: u8 = '.' as u8;

fn num_occupied_at_steady_state(initial_grid: &Vec<u8>, graph: Vec<Option<Vec<usize>>>, occupied_limit: usize) -> usize {
    let mut grid = initial_grid.clone();
    loop {
        let mut change = false;  
        let n = graph.iter().enumerate().map(|(i,value)| {   
            if let Some(c) = value {
                let num_occupied_neighbors = c.iter().filter(|&i| grid[*i] == OCCUPIED).count();
                match grid[i] {
                    OCCUPIED if num_occupied_neighbors >= occupied_limit => { change = true; EMPTY },
                    EMPTY if num_occupied_neighbors == 0 => { change = true; OCCUPIED },
                    _ => grid[i],
                }
            } else {
                FLOOR
            }
        }).collect();
        if !change { return grid.iter().filter(|&t| *t == OCCUPIED).count() }
        grid = n;
    } 
}

fn safe_index(row: i64, column: i64, rows: usize, columns: usize) -> Option<usize> {
    if column < 0 || row < 0 { return None };
    let row = row as usize;
    let column = column as usize;
    if column >= columns || row >= rows { return None }
    Some(column + row*columns)
}

fn get_graph_from_discriminator<F: Fn(i64,i64,usize,usize,&(i64,i64)) -> Option<usize>>(grid: &Vec<u8>,
    rows: usize, columns: usize,
    discriminator: F) -> Vec<Option<Vec<usize>>> {
    let neighbors: [(i64,i64);8] = [(-1,-1), (-1,0),(-1,1),(1,-1), (1,0),(1,1),(0,1),(0,-1)];
    grid.iter().enumerate().map(|(i, c)| {
        match *c {
            FLOOR => None,
            EMPTY | OCCUPIED => { 
                let r = i / columns;
                let c = i - r * columns;
                Some(neighbors.iter().filter_map(|n| discriminator(r as i64,c as i64, rows, columns,n)).collect())
            },
            _ => unreachable!(),
        }
    }).collect()
}

pub fn solve(input: &str) -> Solution {
    
    let cols = input.lines().next().unwrap().len();
    let locations: Vec<u8> = input.chars().filter(|c| !c.is_whitespace()).map(|c| c as u8).collect();
    let rows = locations.len()/cols;

    let connectedness_pt1 = get_graph_from_discriminator(&locations, rows, cols,
        |row: i64, column: i64, rows: usize, columns: usize, n: &(i64,i64)| -> Option<usize> {
        match safe_index(row + n.0, column + n.1, rows, columns) {
            Some(i) if locations[i] != FLOOR => Some(i),
            _ => None,
        }
    });
    let connectedness_pt2 = get_graph_from_discriminator(&locations, rows, cols,
        |row: i64, column: i64, rows: usize, columns: usize, n: &(i64,i64)| -> Option<usize> {
        let mut step: i64 = 1;
        loop {
            match safe_index(row + n.0*step, column + n.1*step, rows, columns) {
                Some(i) => match locations[i] {
                    FLOOR => step += 1,
                    EMPTY | OCCUPIED => return Some(i),
                    _ => unreachable!(),
                },
                _ => return None,
            }
        }
    });


    let p1 = num_occupied_at_steady_state(&locations, connectedness_pt1, 4);
    let p2 = num_occupied_at_steady_state(&locations, connectedness_pt2, 5);
    
    Solution::new(p1,p2)
}
