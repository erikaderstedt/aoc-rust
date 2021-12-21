// https://adventofcode.com/2021/day/21
use crate::common::Solution;

const P1_STARTING_POSITION: usize = 1;
const P2_STARTING_POSITION: usize = 6;

struct HundredSidedDie {
    next_number: usize,
    total_rolls: usize,
}

impl HundredSidedDie {
    fn roll(&mut self) -> usize {
        let d = self.next_number;
        self.next_number += 1;
        if self.next_number == 101 { self.next_number = 1; }
        self.total_rolls += 1;
        d
    }

    fn new() -> HundredSidedDie { HundredSidedDie { next_number: 1, total_rolls: 0 }}
}

struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn take_turn_and_check_win_condition(&mut self, die: &mut HundredSidedDie) -> bool {
        const PT1_WIN_SCORE: usize = 1000;

        let r1 = die.roll();
        let r2 = die.roll();
        let r3 = die.roll();
        self.position += r1 + r2 + r3;
        while self.position > 10 { self.position -= 10; }
        self.score += self.position;
        self.score >= PT1_WIN_SCORE
    }
}

// Count how many paths there are at which turn and how many of those lead to a win
fn solitaire(turn: usize, points: usize, remaining_points_to_win: usize, previous_total_paths: usize, steps: &mut [(usize, usize)])  {
    for (rollsum, frequency) in [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1) ] {
        let points = if points + rollsum > 10 { points + rollsum - 10 } else { points + rollsum };
        let total_paths = frequency * previous_total_paths;
        if points >= remaining_points_to_win {
            steps[turn].0 += total_paths;
        } else {
            steps[turn].1 += total_paths;
            solitaire(turn + 1, points, remaining_points_to_win - points, total_paths, steps);
        }
    }
}

pub fn solve(_input: &str) -> Solution {
    
    let m1 = { 
        let mut player1 = Player { position: P1_STARTING_POSITION, score: 0 };
        let mut player2 = Player { position: P2_STARTING_POSITION, score: 0 };
        let mut die = HundredSidedDie::new();

        loop {
            if player1.take_turn_and_check_win_condition(&mut die) { break; }
            if player2.take_turn_and_check_win_condition(&mut die) { break; }
        }

        usize::min(player1.score, player2.score) * die.total_rolls
    };

    let m2 = {
        const POINTS: usize = 21;
        // Max number of turns is lower than 21. 10 works for my input, I'll put 20 to be safe. It is not performance-critical.
        const MAXIMUM_NUMBER_OF_TURNS: usize = 11;
        let mut p1steps = [(0, 0); MAXIMUM_NUMBER_OF_TURNS];
        let mut p2steps = [(0, 0); MAXIMUM_NUMBER_OF_TURNS];
        solitaire(0, P1_STARTING_POSITION, POINTS, 1, &mut p1steps);
        solitaire(0, P2_STARTING_POSITION, POINTS, 1, &mut p2steps);
        let p1wins: usize = p1steps[0..MAXIMUM_NUMBER_OF_TURNS].iter().skip(1)
                                .zip(p2steps[0..MAXIMUM_NUMBER_OF_TURNS].iter())
                                .map(|(p1,p2)| p1.0 * p2.1) // Combination of p1 wins and p2 !wins at this turn.
                                .sum();
        let p2wins: usize = p2steps[0..MAXIMUM_NUMBER_OF_TURNS].iter()
                                .zip(p1steps[0..MAXIMUM_NUMBER_OF_TURNS].iter())
                                .map(|(p1,p2)| p2.0 * p1.1) // Combination of p2 wins and p1 !wins at this turn.
                                .sum();
        usize::max(p1wins, p2wins)
    };

    Solution::new(m1,m2)
}
