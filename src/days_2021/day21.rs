// https://adventofcode.com/2021/day/21
use crate::common::Solution;

struct Die {
    next_number: usize,
    total_rolls: usize,
}

impl Die {
    fn roll(&mut self) -> usize {
        let d = self.next_number;
        self.next_number += 1;
        if self.next_number == 101 { self.next_number = 1; }
        self.total_rolls += 1;
        d
    }

    fn new() -> Die { Die { next_number: 1, total_rolls: 0 }}
}

const P1_STARTING_POSITION: usize = 1;
const P2_STARTING_POSITION: usize = 6;

struct GameState {
    p1: u8,
    p1_score: u16,
    p2: u8,
    p2_score: u16,
    p1s_turn: bool,
}

impl GameState {

    fn initial() -> GameState {
        GameState {
            p1: P1_STARTING_POSITION as u8,
            p2: P2_STARTING_POSITION as u8,
            p1_score: 0,
            p2_score: 0,
            p1s_turn: true,
        }
    }

    fn advance(&mut self, die: &mut Die) {
        let r1 = die.roll();
        let r2 = die.roll();
        let r3 = die.roll();
        let start = if self.p1s_turn { self.p1 } else { self.p2 };
        let mut p = (start as usize) + r1 + r2 + r3;
        while p > 10 { p -= 10; }
        if self.p1s_turn {
            self.p1 = p as u8;
            self.p1_score += p as u16;
        } else {
            self.p2 = p as u8;
            self.p2_score += p as u16;
        }
        self.p1s_turn = !self.p1s_turn;
    }

    fn is_finished(&self) -> bool { self.p1_score >= 1000 || self.p2_score >= 1000 }

}


fn pack_state(a: u8, b: u8, p1: u8, p2: u8) -> usize {
    (a as usize) * 32*16*16 + 
    (b as usize) * 16*16 +
    (p1 as usize) * 16 +
    (p2 as usize)
}

fn unpack_state(s: usize) -> (u8, u8, u8, u8) {
   ((s >> (5+4+4)) as u8,
    ((s >> (4+4)) & 0b11111) as u8,
    ((s >> 4) & 0b1111) as u8,
    (s & 0b1111) as u8)
}


fn part2(state: usize, cache: &mut [Option<(usize, usize)>; 32*32*16*16]) -> (usize, usize) {
    let (a,b,p1,p2) = unpack_state(state);

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for (occurrences, total_roll) in [(1,3), (3,4), (6,5), (7,6), (6,7), (3, 8), (1,9)] {
        let n = p1 + total_roll;
        let moved = if n > 10 { n - 10 } else { n };
        if moved >= a {
            p1_wins += occurrences;
        } else {
            let new_state = pack_state(b, a - moved, p2, moved);
            let wins = cache[new_state].unwrap_or(part2(new_state, cache));
            p1_wins += wins.1*occurrences;
            p2_wins += wins.0*occurrences;
        }
    }
    cache[state] = Some((p1_wins,p2_wins));

    (p1_wins, p2_wins)
}

pub fn solve(_input: &str) -> Solution {
    
    let m1 = { 
        let mut state = GameState::initial();
        let mut die = Die::new();

        loop {
            state.advance(&mut die);
            if state.is_finished() { break; }
        }

        usize::min(state.p1_score as usize, state.p2_score as usize) * die.total_rolls
    };

    let mut cache = [None; 32*32*16*16];
    let w = part2(pack_state(21, 21, P1_STARTING_POSITION as u8, P2_STARTING_POSITION as u8), &mut cache);
    let m2 = usize::max(w.0, w.1);

    Solution::new(m1,m2)
}

