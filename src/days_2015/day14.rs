// https://adventofcode.com/2015/day/14

use crate::common::Solution;
use regex::Regex;

struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,

    score: usize,
    distance: usize,
}

const TIME: usize = 2503;

pub fn solve(input: &str) -> Solution {
    let r = Regex::new(
        "^\\w+ can fly (\\d+) km/s for (\\d+) seconds, but then must rest for (\\d+) seconds.",
    )
    .unwrap();

    let mut reindeer: Vec<Reindeer> = input
        .lines()
        .map(|line| {
            let (_, [speed, duration, rest]) = r.captures(line).unwrap().extract();
            Reindeer {
                speed: speed.parse::<usize>().unwrap(),
                duration: duration.parse::<usize>().unwrap(),
                rest: rest.parse::<usize>().unwrap(),
                score: 0,
                distance: 0,
            }
        })
        .collect();

    for t in 0..TIME {
        for r in reindeer.iter_mut() {
            let period = r.duration + r.rest;
            if t % period < r.duration {
                r.distance += r.speed;
            }
        }
        // Check which one is in front and increase the score.
        // For ties each get a point.
        let furthest = reindeer.iter().map(|r| r.distance).max().unwrap();
        for r in reindeer.iter_mut().filter(|r| r.distance == furthest) {
            r.score += 1;
        }
    }

    let p1 = reindeer.iter().map(|r| r.distance).max().unwrap();
    let p2 = reindeer.iter().map(|r| r.score).max().unwrap();

    Solution::new(p1, p2)
}
