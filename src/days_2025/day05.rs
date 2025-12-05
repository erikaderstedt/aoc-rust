// https://adventofcode.com/2025/day/5

use crate::common::{Solution, parsed_from_each_line};

struct IngredientRange {
    start: u64,
    stop: u64,
}

pub fn solve(input: &str) -> Solution {

    let (ingredient_id_ranges, ingredient_ids) = input.split_once("\n\n").unwrap();
    let ingredient_id_ranges: Vec<IngredientRange> = parsed_from_each_line(ingredient_id_ranges);
    let ingredients: Vec<u64> = ingredient_ids.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    
    let p1 = ingredients.iter().filter(|i|
        ingredient_id_ranges.iter().any(|r| r.in_range(**i))
    ).count();

    let mut ranges: Vec<IngredientRange> = Vec::new();
    for n in ingredient_id_ranges.into_iter() {
        // is n already covered.
        if ranges.iter().any(|r| r.is_other_a_subrange(&n)) {
            continue
        }

        // is there another range covered by other, if so, remove those
        ranges = ranges.into_iter().filter(|r| !n.is_other_a_subrange(&r)).collect();

        // crop other according to existing ranges, then add it to the list
        for range in ranges.iter_mut() {
            range.intersect(&n);   
        }
        ranges.push(n);
    }

    let p2: u64 = ranges.iter().map(|r| r.num_ingredients()).sum();
    
    Solution::new(p1, p2)
}

impl IngredientRange {
    fn in_range(&self, ingredient_id: u64) -> bool {
        self.start <= ingredient_id && ingredient_id <= self.stop
    }

    fn is_other_a_subrange(&self, other: &IngredientRange) -> bool {
        self.in_range(other.start) && self.in_range(other.stop)
    }

    fn intersect(&mut self, other: &IngredientRange) {
        if other.stop >= self.start && other.start <= self.start {
            self.start = other.stop + 1            
        } else if other.start <= self.stop && other.stop >= self.stop {
            self.stop = other.start - 1;
        }
    }

    fn num_ingredients(&self) -> u64 {
        self.stop - self.start + 1
    }
}

impl std::str::FromStr for IngredientRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((start, stop)) => {
                match (start.parse::<u64>(), stop.parse::<u64>()) {
                    (Ok(start), Ok(stop)) => Ok(Self { start, stop }),
                    _ => Err("unable to parse"),
                }
            },
             _ => Err("No dash!"),
        } 
    }
}
