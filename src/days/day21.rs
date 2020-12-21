use crate::common::Solution;
use itertools::Itertools;
use std::collections::{HashSet,BTreeMap};

struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Food {
        let i = s.find('(').unwrap();
        Food { 
            ingredients: s[..(i-1)].split(' ').collect(), 
            allergens: s[(i+10)..(s.len()-1)].split(", ").collect() 
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let foods: Vec<Food> = input.lines().map(|s| Food::parse(s)).collect();

    let mut unknown_allergens: HashSet<&str> = foods.iter().map(|f| f.allergens.iter()).flatten().cloned().collect();
    let mut known_allergens: BTreeMap<&str,&str> = BTreeMap::new();
    let mut matched_ingredients: HashSet<&str> = HashSet::new();
    
    while unknown_allergens.len() > 0 {
        unknown_allergens = unknown_allergens.into_iter().filter(|u| {
            // Take the intersection of the ingredients list where this allergen occurs
            let foods_with_this_allergen: Vec<&Food> = foods.iter().filter(|f| f.allergens.contains(u)).collect();
            assert!(foods_with_this_allergen.len() > 0);
            let candidate_ingredients:Vec<&str> = foods_with_this_allergen.iter()
                .skip(1)
                .fold(foods_with_this_allergen[0].ingredients.clone(),|i,f| i.intersection(&f.ingredients).cloned().collect())
                .difference(&matched_ingredients)
                .cloned()
                .collect();
            
            if candidate_ingredients.len() == 1 {
                known_allergens.insert(u, candidate_ingredients[0]);
                matched_ingredients.insert(candidate_ingredients[0]);
                false
            } else {
                true
            }
        }).collect();
    }
    let p1 = foods.iter()
        .map(|food| food.ingredients
            .difference(&matched_ingredients)
            .count())
        .sum::<usize>();
        
    let p2 = known_allergens.values().join(",");

    Solution::new(p1,p2)
}