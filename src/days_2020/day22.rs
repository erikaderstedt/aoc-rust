use crate::common::Solution;
use std::collections::HashSet;
use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;

fn score(cards: &Vec<usize>) -> usize {
    cards.iter().rev().enumerate().map(|(i, c)| (i+1) * (*c)).sum::<usize>()
}

fn play_combat(player1: &Vec<usize>, player2: &Vec<usize>) -> usize {
    let mut player1 = player1.clone();
    let mut player2 = player2.clone();
    loop {
        let c1 = player1.remove(0);
        let c2 = player2.remove(0);
        if c1 > c2 {
            player1.push(c1);
            player1.push(c2);
        } else {
            player2.push(c2);
            player2.push(c1);
        }

        if player1.is_empty() { return score(&player2); }
        if player2.is_empty() { return score(&player1); }
    }
}

enum Player { Player1,Player2 }

fn play_recursive_combat(mut player1: Vec<usize>, mut player2: Vec<usize>) -> (usize,Player) {
    let mut previous: HashSet<u64> = HashSet::new();
    loop {
        let mut hasher = DefaultHasher::new();
        player1.hash(&mut hasher);
        player2.hash(&mut hasher);
        let h = hasher.finish();
        if !previous.insert(h) { return (0, Player::Player1); }
        let c1 = player1.remove(0);
        let c2 = player2.remove(0);
        let winner = if player1.len() >= c1 && player2.len() >= c2 {
            play_recursive_combat(player1[..c1].to_vec(), player2[..c2].to_vec()).1
        } else if c1 > c2 {
            Player::Player1
        } else {
            Player::Player2
        };
        match winner {
            Player::Player1 => {
                player1.push(c1);
                player1.push(c2);
            },
            Player::Player2 => {
                player2.push(c2);
                player2.push(c1);
            },
        }

        if player1.is_empty() { return (score(&player2), Player::Player2); }
        if player2.is_empty() { return (score(&player1), Player::Player1); }
    }
}

pub fn solve(input: &str) -> Solution {
    let player1: Vec<usize> = input.lines().skip(1).take_while(|&c| c != "").map(|l| l.parse::<usize>().unwrap()).collect();
    let player2 :Vec<usize> = input.lines().skip_while(|&c| c != "Player 2:").skip(1).map(|l| l.parse::<usize>().unwrap()).collect();

    let p1 = play_combat(&player1, &player2);
    let p2 = play_recursive_combat(player1, player2).0;
    
    Solution::new(p1,p2)
}
