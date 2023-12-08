// https://adventofcode.com/2023/day/7

use itertools::Itertools;

use crate::common::Solution;

fn evaluate_line(line: &str, use_jokers: bool) -> Hand {
    let bid = line.split(' ').last().unwrap().parse::<usize>().unwrap();

    let mut h: usize = 0;
    let mut cards = [0usize; 5];
    let mut num_each = [0usize; 15];
    for (i,c) in line.as_bytes().iter().enumerate().take(5) {
        let x = match c {
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            b'T' => 10,
            b'J' => {
                if use_jokers {
                    1
                } else {
                    11
                }
            }
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => panic!("unkown"),
        };
        cards[i] = x;
        h |= 1 << x;
        num_each[x] += 1;
    }

    let num_jokers = num_each[1];
    let exactly_three_of_one_kind = *num_each.iter().max().unwrap() == 3;

    let hand_type = match h.count_ones() {
        1 => HandType::FiveOfAKind,
        2 if num_jokers > 0 => HandType::FiveOfAKind,
        2 if exactly_three_of_one_kind => HandType::FullHouse,
        2 => HandType::FourOfAKind,
        5 if num_jokers > 0 => HandType::OnePair,
        5 => HandType::HighCard,
        4 if num_jokers == 0 => HandType::OnePair,
        4 => HandType::ThreeOfAKind,
        3 => {
            match num_jokers {
                2 | 3 => HandType::FourOfAKind, // 3 1 1 or 2 2 1
                1 => if exactly_three_of_one_kind {
                    HandType::FourOfAKind // 1 1 3
                } else {
                    HandType::FullHouse // 1 2 2 
                },
                0 => if exactly_three_of_one_kind {
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
