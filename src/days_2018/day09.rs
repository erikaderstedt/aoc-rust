// https://adventofcode.com/2018/day/9

use std::collections::VecDeque;

use crate::common::Solution;
use itertools::Itertools;

fn play_game(n_players: &usize, last_marble_worth: &usize) -> usize {
    let mut circle: VecDeque<usize> = VecDeque::with_capacity(last_marble_worth * 100);
    circle.push_back(0);
    let mut scores = vec![0; *n_players];

    // rotate right = move pointing arrow counter-clockwise
    // rotate left = move pointing arrow clockwise
    // last element is current marble
    for marble in 1..=*last_marble_worth {
        if marble.rem_euclid(23) == 0 {
            // In addition, the marble 7 marbles counter-clockwise from the current marble
            // is removed from the circle and also added to the current player's score.
            // The marble located immediately clockwise of the marble that was removed
            // becomes the new current marble.
            let player = marble.rem_euclid(*n_players);
            circle.rotate_right(7);
            scores[player] = scores[player] + marble + circle.pop_back().unwrap_or(0);
            circle.rotate_left(1);
        } else {
            // placing the lowest-numbered remaining marble into the circle between
            //  the marbles that are 1 and 2 marbles clockwise of the current marble.
            //  (When the circle is large enough, this means that there is one
            // marble between the marble that was just placed and the current marble.)
            // The marble that was just placed then becomes the current marble.
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    scores.into_iter().max().unwrap_or(0)
}

pub fn solve(input: &str) -> Solution {
    let (n_players, last_marble_worth) = input
        .split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple()
        .unwrap();

    let p1 = play_game(&n_players, &last_marble_worth);
    let p2 = play_game(&n_players, &(last_marble_worth * 100));

    Solution::new(p1, p2)
}
