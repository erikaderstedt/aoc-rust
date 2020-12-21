use crate::common::Solution;
use itertools::Itertools;

#[derive(Debug)]
struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
pub fn solve(input: &str) -> Solution {
    let claims: Vec<Claim> = input.lines().map(|line| -> Claim {
        let i = line.find('@').unwrap();
        let id = line[1..(i-1)].parse::<u16>().unwrap();
        let j = line.find(':').unwrap();
        let (x,y) = line[(i+2)..j].split(',').map(|v| v.parse::<u16>().unwrap()).collect_tuple().unwrap();
        let (width,height) = line[(j+2)..].split('x').map(|v| v.parse::<u16>().unwrap()).collect_tuple().unwrap();
        Claim { id, x, y, width, height }
    }).collect();

    let mut squares = [0u16;WIDTH*HEIGHT];
    for c in claims.iter() {
        for u in c.iter() { squares[u] += 1; }
    }
    let p1 = squares.iter().filter(|&v| v >= &2).count();

    let non_overlapping_claim = claims.iter().filter(|c| c.iter().all(|u| squares[u] == 1)).next().unwrap();

    Solution::new(p1, non_overlapping_claim.id)
}

impl Claim {
    fn iter(&self) -> ClaimIterator { 
        ClaimIterator { x: self.x, y: self.y, min_x: self.x, max_x: self.x + self.width - 1, max_y: self.y + self.height - 1 } 
    }
}

struct ClaimIterator { x: u16, y: u16, min_x: u16, max_x: u16, max_y: u16 } 

impl Iterator for ClaimIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.y > self.max_y { None } else {
            let i = WIDTH*(self.y as usize) + (self.x as usize);
            self.x += 1;
            if self.x > self.max_x { self.x = self.min_x; self.y += 1 }
            Some(i)
        }
    }
}