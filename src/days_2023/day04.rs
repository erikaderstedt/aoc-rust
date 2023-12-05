// https://adventofcode.com/2023/day/4

use std::str::FromStr;
use crate::common::{Solution, parsed_from_each_line};

struct Card {
    id: usize,
    winning_numbers: u128,
    numbers: u128,
}

impl Card {
    fn number_of_matches(&self) -> usize {
        (self.winning_numbers & self.numbers).count_ones() as usize
    }

    fn value(&self) -> usize {
        match self.number_of_matches() {
            0 => 0,
            x => 2usize.pow((x as u32) - 1),
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s[5..8].trim_start().parse::<usize>().map_err(|_| "Invalid card ud.")?;
        let mut winning_numbers = 0;
        let mut numbers = 0;
        for i in 0..10 {
            winning_numbers |= 1 << s[(10+3*i)..(12+3*i)].trim_start().parse::<u32>().map_err(|_| "Invalid winning number.")?;
        }
        for i in 0..25 {
            numbers |= 1 << s[(42+3*i)..(44+3*i)].trim_start().parse::<u32>().map_err(|_| "Invalid scratch card number.")?;
        }

        Ok( Card { id, winning_numbers, numbers})
    }
}

pub fn solve(input: &str) -> Solution {    
    let cards: Vec<Card> = parsed_from_each_line(input);    

    let p1: usize = cards.iter().map(|c| c.value()).sum();

    let mut num_duplicates = vec![1usize; cards.len()];
    for card in cards.iter() {
        let n = num_duplicates[card.id-1];
        for i in card.id..(card.id+card.number_of_matches()) {
            if i < cards.len() {
                num_duplicates[i] += n * 1; 
            }
        }
    }
    let p2: usize = num_duplicates.iter().sum();

    Solution::new(p1,p2)
}
