// https://adventofcode.com/2025/day/5

use crate::common::{Solution, parsed_from_each_line};

#[derive(Debug)]
struct IngredientRange {
    start: i64,
    stop: i64,
}

pub fn solve(input: &str) -> Solution {

    let (ingredient_id_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();
    let mut ingredient_id_ranges: Vec<IngredientRange> = parsed_from_each_line(ingredient_id_ranges);
    let ingredients: Vec<i64> = ingredient_ids.lines().map(|line| line.parse::<i64>().unwrap()).collect();
    
    let p1 = ingredients.iter().filter(|i|
        ingredient_id_ranges.iter().any(|r| r.in_range(**i))
    ).count();

    ingredient_id_ranges.sort_unstable_by(|r1,r2| {
        if r1.start == r2.start {
            r1.stop.cmp(&r2.stop)
        } else {
            r1.start.cmp(&r2.start)
        }
    });

    let mut b = IngredientRange { start: 0, stop: -1 };
    let mut p2 = 0;
    for s in ingredient_id_ranges.iter() {
        if b.stop < s.start {
            p2 = p2 + b.num_ingredients();
            b.start = s.start;
            b.stop = s.stop;
        } else {
            b.stop = std::cmp::max(b.stop, s.stop);
        }
    }
    p2 = p2 + b.num_ingredients();

    Solution::new(p1, p2)
}

impl IngredientRange {
    fn in_range(&self, ingredient_id: i64) -> bool {
        self.start <= ingredient_id && ingredient_id <= self.stop
    }

    fn num_ingredients(&self) -> i64 {
        self.stop - self.start + 1
    }
}

impl std::str::FromStr for IngredientRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((start, stop)) => {
                match (start.parse::<i64>(), stop.parse::<i64>()) {
                    (Ok(start), Ok(stop)) => Ok(Self { start, stop }),
                    _ => Err("unable to parse"),
                }
            },
             _ => Err("No dash!"),
        } 
    }
}
