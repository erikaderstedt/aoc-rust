// https://adventofcode.com/2021/day/15
use crate::common::Solution;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Cost = u16;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: Cost,
    index: usize,
}

struct Location {
    index: usize,
    x: usize,
    y: usize
}

fn neighbors<const N:usize>(index: usize) -> [Option<Location>;4] {
    let y = index / N;
    let x = index - y * N;
   [if y > 0     { Some(Location { index: index - N, x, y: y-1 }) } else { None },
    if x > 0     { Some(Location { index: index - 1, x: x-1, y }) } else { None },
    if x < N - 1 { Some(Location { index: index + 1, x: x+1, y }) } else { None },
    if y < N - 1 { Some(Location { index: index + N, x, y: y+1 }) } else { None }]
}

fn find_shortest_path<const N:usize>(input: &str) -> Option<Cost> {
    let end = N*N-1;
    let mut grid = [[0u8;N];N];
	for (y,line) in input.lines().enumerate() {
		for (x,c) in line.as_bytes().iter().enumerate() {
            grid[y][x] = c - b'0';
		}
	}
    let orig_size = input.lines().count();
    let repeats = grid.len()/orig_size;

    for y0 in 0..repeats {
        for x0 in 0..repeats {
            if x0 == 0 && y0 == 0 { continue; }
            for y in 0..orig_size {
                for x in 0..orig_size {
                    grid[y0*orig_size+y][x0*orig_size+x] = (grid[y][x] + (x0 as u8) + (y0 as u8) - 1) % 9 + 1;
                }
            }
        }
    }

    let mut dist: Vec<_> = (0..N*N).map(|_| Cost::MAX).collect();
    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, index: 0 });
    dist[0] = 0;
    
    while let Some(State { cost, index }) = heap.pop() {
        if index == end { return Some(cost) }
        if cost > dist[index] { continue; }
        heap.extend(
            neighbors::<N>(index).iter().filter_map(|n| 
            match n {
                Some(Location { index: other_index,x,y }) => {
                    let next = State { cost: cost + grid[*y][*x] as Cost, index: *other_index };
                    if next.cost < dist[next.index] {
                        dist[next.index] = next.cost;
                        Some(next)
                    } else {
                        None
                    }
                },
                None => None,
            }));
    }
    None
}

pub fn solve(input: &str) -> Solution {
	
	let m1 = find_shortest_path::<100>(input).unwrap();
	let m2 = find_shortest_path::<500>(input).unwrap();
    Solution::new(m1,m2)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
