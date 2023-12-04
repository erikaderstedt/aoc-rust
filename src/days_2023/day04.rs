// https://adventofcode.com/2023/day/4

use std::{collections::HashSet, str::FromStr};
use crate::common::{Solution, parsed_from_each_line};

struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>, // 
}

impl Card {
    fn number_of_matches(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn value(&self) -> usize {
        match self.number_of_matches() {
            1 => 1,
            0 => 0,
            x => 2usize.pow((x as u32) - 1),
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.split(":").next().unwrap().split(" ").last().unwrap().parse::<usize>().map_err(|_| "Invalid game number.")?;
        let winning_numbers = s.split(":").last().unwrap().split("|").next()
            .unwrap().split(" ")
            .filter_map(|q| q.parse::<usize>().ok()).collect();
        let numbers = s.split(":").last().unwrap().split("|").last()
        .unwrap().split(" ")
        .filter_map(|q| q.parse::<usize>().ok()).collect();

        Ok( Card { id, winning_numbers, numbers})
    }
}

pub fn solve(input: &str) -> Solution {    
    let cards: Vec<Card> = parsed_from_each_line(input);    

    let p1: usize = cards.iter().map(|c| c.value()).sum();
    
    let mut num_duplicates = vec![1usize; cards.len()];
    for card in cards.iter() {
        for i in (card.id)..(card.id+card.number_of_matches()) {
            if i < cards.len() {
                num_duplicates[i] += num_duplicates[card.id-1] * 1; 
            }
        }
    }
    let p2: usize = num_duplicates.iter().sum();

    Solution::new(p1,p2)
}
