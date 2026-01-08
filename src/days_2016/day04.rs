// https://adventofcode.com/2016/day/4

use crate::common::{parsed_from_each_line, Solution};
use itertools::Itertools;
use std::str::FromStr;

struct Room {
    encrypted_name: Vec<u8>,
    sector_id: u32,
    checksum: [u8; 5],
}

const DASH: u8 = '-' as u8;
const A: u8 = 'a' as u8;
const LETTERS_IN_ALPHABET: u32 = 26;
const NORTHPOLE_OBJECTS_STORAGE: &str = "northpole object storage";

impl Room {
    fn checksum_ok(&self) -> bool {
        self.encrypted_name
            .iter()
            .filter(|v| **v != DASH)
            .sorted()
            .dedup_with_count()
            .sorted_by_key(|a| usize::MAX - a.0)
            .take(5)
            .zip(self.checksum.iter())
            .all(|((_, v1), v2)| *v1 == *v2)
    }

    fn decipher(&self) -> String {
        self.encrypted_name
            .iter()
            .map(|c| match *c {
                DASH => ' ',
                x => {
                    ((((((x as u32) - (A as u32)) + self.sector_id) % LETTERS_IN_ALPHABET) as u8)
                        + A) as char
                }
            })
            .collect()
    }
}

pub fn solve(input: &str) -> Solution {
    let rooms: Vec<Room> = parsed_from_each_line(input);

    let p1: u32 = rooms
        .iter()
        .filter(|room| room.checksum_ok())
        .map(|r| r.sector_id)
        .sum();

    let p2 = rooms
        .iter()
        .find(|room| room.decipher() == NORTHPOLE_OBJECTS_STORAGE)
        .unwrap()
        .sector_id;

    Solution::new(p1, p2)
}

impl FromStr for Room {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once('[').unwrap();
        let sector_id = s1.split('-').last().unwrap();
        let encrypted_name = s1[0..(s1.len() - sector_id.len() - 1)]
            .as_bytes()
            .iter()
            .cloned()
            .collect();
        let sector_id = sector_id.parse::<u32>().unwrap();
        let checksum: [u8; 5] = s2
            .as_bytes()
            .iter()
            .take(5)
            .cloned()
            .collect_array()
            .unwrap();
        Ok(Room {
            encrypted_name,
            sector_id,
            checksum,
        })
    }
}
