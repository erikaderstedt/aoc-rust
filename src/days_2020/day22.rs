use crate::common::Solution;
use std::collections::HashSet;
use seahash::hash;

fn score(cards: &Vec<u8>) -> usize {
    cards.iter().rev().enumerate().map(|(i, c)| (i+1) * (*c as usize)).sum::<usize>()
}

fn play_combat(player1: &Vec<u8>, player2: &Vec<u8>) -> usize {
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

enum Result { Player1, Player2 }

fn play_recursive_combat(player1: &mut Vec<u8>, player2: &mut Vec<u8>) -> Result {
    let mut previous: HashSet<u64> = HashSet::new();
    loop {
        let h = hash(&player1[..]) * hash(&player2[..]);
        if !previous.insert(h) { return Result::Player1; }
        let c1 = player1.remove(0) as usize;
        let c2 = player2.remove(0) as usize;
        let result = if player1.len() >= c1 && player2.len() >= c2 {
            let mut p1_clone = player1.iter().take(c1).cloned().collect();
            let mut p2_clone = player2.iter().take(c2).cloned().collect();
            play_recursive_combat(&mut p1_clone, &mut p2_clone)
        } else if c1 > c2 {
            Result::Player1
        } else {
            Result::Player2
        };
        match result {
            Result::Player1 => {
                player1.push(c1 as u8);
                player1.push(c2 as u8);
            },
            Result::Player2 => {
                player2.push(c2 as u8);
                player2.push(c1 as u8);
            },
        }

        if player1.is_empty() { return Result::Player2; }
        if player2.is_empty() { return Result::Player1; }
    }
}

pub fn solve(input: &str) -> Solution {
    let mut player1: Vec<u8> = input.lines().skip(1).take_while(|&c| c != "").map(|l| l.parse::<u8>().unwrap()).collect();
    let mut player2 :Vec<u8> = input.lines().skip_while(|&c| c != "Player 2:").skip(1).map(|l| l.parse::<u8>().unwrap()).collect();

    let p1 = play_combat(&player1, &player2);
    let winner_p2 = play_recursive_combat(&mut player1, &mut player2);
    let p2 = score(match winner_p2 { Result::Player1 => &player1, Result::Player2 => &player2 });
    
    Solution::new(p1,p2)
}
