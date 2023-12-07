// https://adventofcode.com/2023/day/7

use itertools::Itertools;

use crate::common::Solution;

fn evaluate_line(line: &str, use_jokers: bool) -> Hand {
    let bid = line.split(' ').last().unwrap().parse::<usize>().unwrap();

    let mut h: usize = 0;
    let mut i = 0;
    let mut cards = [0usize; 5];
    let mut num_each = [0usize; 15];
    for c in line.chars().take(5) {
        cards[i] = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => {
                if use_jokers {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unkown"),
        };
        h |= 1 << (cards[i] as usize);
        num_each[cards[i] as usize] += 1;
        i += 1;
    }

    let num_jokers = num_each[1];
    let highest_count = num_each.iter().max().unwrap().clone();

    let hand_type = match h.count_ones() {
        1 => HandType::FiveOfAKind,
        2 => {
            if num_jokers > 0 {
                HandType::FiveOfAKind
            } else if highest_count == 3 {
                HandType::FullHouse
            } else {
                HandType::FourOfAKind
            }
        }
        5 => {
            if num_jokers > 0 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
        4 => {
            if num_jokers == 0 {
                HandType::OnePair
            } else {
                HandType::ThreeOfAKind
            }
        }
        3 => {
            match num_jokers {
                2 | 3 => HandType::FourOfAKind, // 3 1 1 or 2 2 1
                1 => if highest_count == 3 {
                    HandType::FourOfAKind // 1 1 3
                } else {
                    HandType::FullHouse // 1 2 2 
                },
                0 => if highest_count == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                },
                _ => panic!("Can't have more than 3 jokers if there are 3 types of cards in a hand of five."),
            }
        }
        _ => panic!("Not between 1 and 5 types of cards?"),
    };

    Hand {
        hand_type,
        cards,
        bid,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: [usize; 5], // Needed to compare equal hands
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            x => x,
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let p1: usize = input
        .lines()
        .map(|line| evaluate_line(line, false))
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();

    let p2: usize = input
        .lines()
        .map(|line| evaluate_line(line, true))
        .sorted()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum();

    Solution::new(p1, p2)
}
