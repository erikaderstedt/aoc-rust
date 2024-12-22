// https://adventofcode.com/2024/day/21

use std::str::from_utf8;

use crate::common::Solution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Numeric {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

#[derive(Clone, PartialEq, Eq, Hash,Copy)]
enum Directional {
    Left,
    Right,
    Up,
    Down,
    A,
}

impl Directional {
    fn from_str(s: &str) -> Vec<Directional> {
        s.bytes().map(|x| match x {
            b'<' => Directional::Left,
            b'>' => Directional::Right,
            b'^' => Directional::Up,
            b'v' => Directional::Down,
            b'A' => Directional::A,
            _ => panic!("unknown directional")
        }).collect()
    }
}

impl std::fmt::Display for Directional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Directional::A => "A",
            Directional::Down => "v",
            Directional::Left => "<",
            Directional::Right => ">",
            Directional::Up => "^",
        })
    }
}

fn expand(mut d: Vec<Directional>, depth: usize) -> Vec<Directional> {
    d.insert(0, Directional::A);
    let replacement: Vec<Directional> = d.windows(2).map(|pair| {
        // How to (optimally) get from the first to the second, and append an A
        // The missing states are not required to move horizontally or vertically on the numeric
        // keypad.
        match (pair[0], pair[1]) {
            (Directional::Up, Directional::A) => vec![Directional::Right, Directional::A],
            (Directional::Up, Directional::Right) => vec![Directional::Down, Directional::Right, Directional::A],
            (Directional::Up, Directional::Up) => vec![Directional::A],
            (Directional::Left, Directional::A) => vec![Directional::Right, Directional::Right, Directional::Up, Directional::A],
            (Directional::Left, Directional::Down) => vec![Directional::Right, Directional::A],
            (Directional::Left, Directional::Left) => vec![Directional::A],
            (Directional::Left, Directional::Up) => vec![Directional::Right, Directional::Up, Directional::A],
            (Directional::Right, Directional::A) => vec![Directional::Up, Directional::A],
            (Directional::Right, Directional::Up) => vec![Directional::Left, Directional::Up, Directional::A],
            (Directional::Right, Directional::Right) => vec![Directional::A],
            (Directional::Down, Directional::A) => vec![Directional::Up, Directional::Right, Directional::A],
            (Directional::Down, Directional::Left) => vec![Directional::Left, Directional::A],
            (Directional::Down, Directional::Right) => vec![Directional::Right, Directional::A],
            (Directional::Down, Directional::Down) => vec![Directional::A],
            (Directional::A, Directional::Up) => vec![Directional::Left, Directional::A],
            (Directional::A, Directional::Left) => vec![Directional::Down, Directional::Left, Directional::Left, Directional::A],
            (Directional::A, Directional::Down) => vec![Directional::Left, Directional::Down, Directional::A],
            (Directional::A, Directional::Right) => vec![Directional::Down, Directional::A],
            (Directional::A, Directional::A) => vec![Directional::A],
            (a,b) => panic!("This state pair should not be needed: {} -> {}", a, b),
        }
    }).flatten().collect();

    if depth > 0 {
        expand(replacement, depth - 1)
    } else {
        replacement
    }
}


fn complexity_for_entering_keycode<const N: usize>(keycode: &str) -> usize {
    let complexity = from_utf8(&keycode.as_bytes()[0..3])
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let states: Vec<Numeric> = vec![Numeric::A].into_iter().chain(Numeric::parse_string(&keycode).into_iter()).collect();
    let length = states.windows(2).map(|v| -> usize {
        let (from_x,from_y) = v[0].coordinate();
        let (to_x, to_y) = v[1].coordinate();

        let total = match (to_x-from_x, from_y - to_y) {
            (0,1) => Directional::from_str("<A>A"),
            (0,2) => Directional::from_str("<AA>A"),
            (0,3) => Directional::from_str("<AAA>A"),
            (0,-1) => Directional::from_str("<vA^>A"),
            (0,-2) => Directional::from_str("<vAA^>A"),
            (0,-3) => Directional::from_str("<vAAA^>A"),
            (-2,0) => Directional::from_str("v<<AA>>^A"),
            (-1,0) => Directional::from_str("v<<A>>^A"),
            (0,0) => vec![],
            (1,0) => Directional::from_str("vA^A"),
            (2,0) => Directional::from_str("vAA^A"),

            (-2,3) => Directional::from_str("<AAAv<AA>>^A"), // must go up first
            (-2,2) if v[0] == Numeric::A => Directional::from_str("<AAv<AA>>^A"), // must go up first
            (-2,1) if v[0] == Numeric::A => Directional::from_str("<Av<AA>>^A"), // must go up first
            (-2,2) => Directional::from_str("v<<AA>^AA>A"),
            (-2,1) => Directional::from_str("v<<AA>^A>A"),
            
            (-2,-1) => Directional::from_str("<vA<AA>>^A"), // could be others
            (-2,-2) => Directional::from_str("<vAA<AA>>^A"), // could be others
            (-2,-3) => panic!("not a valid move"),
            
            (-1,3) if v[0] == Numeric::Zero => Directional::from_str("<AAAv<A>>^A"),
            (-1,2) if v[0] == Numeric::Zero => Directional::from_str("<AAv<A>>^A"),
            (-1,1) if v[0] == Numeric::Zero => Directional::from_str("<Av<A>>^A"),
            (-1,3) => Directional::from_str("v<<A>^AAA>A"),
            (-1,2) => Directional::from_str("v<<A>^AA>A"),
            (-1,1) => Directional::from_str("v<<A>^A>A"),
            (-1,-1) => Directional::from_str("<vA<A>>^A"),
            (-1,-2) => Directional::from_str("<vAA<A>>^A"),
            (-1,-3) => Directional::from_str("<vAAA<A>>^A"),

            (1,3) => Directional::from_str("vA<^AAA>A"),
            (1,2) => Directional::from_str("vA<^AA>A"), 
            (1,1) => Directional::from_str("vA<^A>A"),
            (1,-1) if v[1] != Numeric::Zero => Directional::from_str("<vA>A^A"),
            (1,-2) if v[1] != Numeric::Zero => Directional::from_str("<vAA>A^A"),
            (1,-3) if v[1] != Numeric::Zero => Directional::from_str("<vAAA>A^A"),
            (1,-1) => Directional::from_str("vA<A^>A"),
            (1,-2) => Directional::from_str("vA<AA^>A"),
            (1,-3) => Directional::from_str("vA<AAA^>A"),

            (2,3) => Directional::from_str("<AAAv>AA^A"), // could be others
            (2,2) => Directional::from_str("<AAv>AA^A"), // could be others
            (2,1) => Directional::from_str("<Av>AA^A"), // could be others
            (2,-1) if v[1] != Numeric::A => Directional::from_str("<vA>AA^A"),
            (2,-2) if v[1] != Numeric::A => Directional::from_str("<vAA>AA^A"),
            (2,-3) if v[1] != Numeric::A => Directional::from_str("<vAAA>AA^A"),
            (2,-1) => Directional::from_str("vAA<A^>A"),
            (2,-2) => Directional::from_str("vAA<AA^>A"),
            (2,-3) => Directional::from_str("vAA<AAA^>A"),

            _ => panic!("unsupported move"),
        };

        let r = expand(total, N-2);

        r.len()
    }).sum::<usize>();

    complexity * length
}

pub fn solve(input: &str) -> Solution {
    let keycodes: Vec<&str> = input.lines().collect();

    let p1 = keycodes
        .iter()
        .map(|keycode| -> usize {complexity_for_entering_keycode::<2>(keycode) })
        .sum::<usize>();

    let p2 = keycodes
        .iter()
        .map(|keycode| -> usize {complexity_for_entering_keycode::<25>(keycode) })
        .sum::<usize>();
    // p2 runtime 9000 seconds!

    Solution::new(p1, p2)
}

impl Numeric {
    fn parse_string(enter: &str) -> Vec<Numeric> {
        enter
            .as_bytes()
            .iter()
            .filter_map(|&u| -> Option<Numeric> {
                match u {
                    b'0' => Some(Numeric::Zero),
                    b'1' => Some(Numeric::One),
                    b'2' => Some(Numeric::Two),
                    b'3' => Some(Numeric::Three),
                    b'4' => Some(Numeric::Four),
                    b'5' => Some(Numeric::Five),
                    b'6' => Some(Numeric::Six),
                    b'7' => Some(Numeric::Seven),
                    b'8' => Some(Numeric::Eight),
                    b'9' => Some(Numeric::Nine),
                    b'A' => Some(Numeric::A),
                    _ => None,
                }
            })
            .collect()
    }

    fn coordinate(&self) -> (isize, isize) {
        match self {
            Self::Seven => (0,0),
            Self::Eight => (1,0),
            Self::Nine => (2,0),
            Self::Four => (0,1),
            Self::Five => (1,1),
            Self::Six => (2,1),
            Self::One => (0,2),
            Self::Two => (1,2),
            Self::Three => (2,2),
            Self::Zero => (1,3),
            Self::A => (2,3),
        }
    }
}
