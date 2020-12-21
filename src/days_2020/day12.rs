use crate::common::Solution;
use std::str::FromStr;
use crate::common::parsed_from_each_line;

#[derive(Debug)]
enum MoveType { North, South, West, East, Turn, Forward, }

#[derive(Debug,Clone)]
enum Facing { North, South, West, East, }

impl Facing {
    fn turn(&self, degrees: i64) -> Facing {
        let t = [Facing::North, Facing::West, Facing::South, Facing::East];
        let i: i64 = match self {
            Facing::North => 0,
            Facing::South => 2,
            Facing::West => 1,
            Facing::East => 3,
        };
        let s = match degrees {
            90 => 1,
            180 => 2,
            270 => 3,
            -90 => -1,
            -180 => -2,
            -270 => -3,
            _ => panic!("unexpected number of degrees {}", degrees),
        };
        t[((i + s + 100) as usize) % 4].clone()
    } 
}

#[derive(Debug)]
struct Move { dir: MoveType, length: i64, }

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut length = s[1..].parse::<i64>().map_err(|_| "Bad number in move.")?;
        let dir = match s.chars().next().unwrap() {
            'N' => MoveType::North,
            'S' => MoveType::South,
            'W' => MoveType::West,
            'E' => MoveType::East,
            'L' => MoveType::Turn,
            'R' => { length = 360 - length; MoveType::Turn},
            'F' => MoveType::Forward,
            _ => panic!("Unexpected move type"),
        };
        Ok(Move { dir, length })
    }
    
}

pub fn solve(input: &str) -> Solution {
    let moves: Vec<Move> = parsed_from_each_line(input);

    let _p1 = moves.iter().fold((0i64,0i64,Facing::East), |(x,y,f), m| {
        let l = m.length;
        match m.dir {
            MoveType::North => (x,y+l,f),
            MoveType::West => (x-l,y,f),
            MoveType::South => (x,y-l,f),
            MoveType::East => (x+l, y,f),
            MoveType::Forward => match f {
                Facing::East => (x+l,y,f),
                Facing::West => (x-l,y,f),
                Facing::North => (x,y+l,f),
                Facing::South => (x,y-l,f),
            },
            MoveType::Turn => (x,y,f.turn(l)),
        }
    });
    let p1 = _p1.0.abs() + _p1.1.abs();

    let _p2 = moves.iter().fold((0i64,0i64,10i64,1i64), |(x,y,wx,wy), m| {
        let l = m.length;
        match m.dir {
            MoveType::North => (x,y,wx,wy+l),
            MoveType::West => (x,y,wx-l,wy),
            MoveType::South => (x,y,wx,wy-l),
            MoveType::East => (x, y,wx+l,wy),
            MoveType::Forward => (x + l*wx, y+l*wy, wx, wy),
            MoveType::Turn if l == 90 => (x,y, -wy,wx),
            MoveType::Turn if l == 180 => (x,y, -wx,-wy),
            MoveType::Turn if l == 270 => (x,y, wy,-wx),
            _ => panic!("Unknown move configuration"),
        }
    });
    let p2 = _p2.0.abs() + _p2.1.abs();
    
    Solution::new(p1,p2)
}