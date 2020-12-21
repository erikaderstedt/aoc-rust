use crate::common::Solution;
use std::collections::{HashSet, HashMap};
use regex::Regex;

#[derive(Debug,Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

pub fn solve(input: &str) -> Solution {
    let food_r = Regex::new(r"^([a-z, ]+) \(contains ([a-z, ]+)\)$").expect("Bad");

    let foods: Vec<Food> = input.lines().map(|line| {
        let cap = food_r.captures_iter(line).next().unwrap();
        let ingredients: HashSet<String> = cap[1].split(' ').map(|x| x.to_string()).collect();
        let allergens: HashSet<String> = cap[2].split(", ").map(|x| x.to_string()).collect();
        Food { ingredients, allergens}
    }).collect();

    // Get all allergens
    let all_allergens:HashSet<String> = foods.iter().map(|food|

        food.allergens.iter().cloned()
    ).flatten().collect();

    let mut unknown_allergens = all_allergens.clone();
    let mut known_allergens: HashMap<String,String> = HashMap::new();

    let mut analyzed_foods = foods.clone();
    while unknown_allergens.len() > 0 {
        let c = unknown_allergens.clone();
        let mut found = false;
        for allergen in c.iter() {
            let foods_with_this_allergen: Vec<&Food> = analyzed_foods.iter().filter(|food| food.allergens.contains(allergen)).collect();
            assert!(foods_with_this_allergen.len() > 0);
            let first = foods_with_this_allergen[0].ingredients.clone();
            let possibilities = foods_with_this_allergen.iter().skip(1).fold(first,|acc, f| {
                acc.intersection(&f.ingredients).cloned().collect()
            });
            if possibilities.len() == 1 {
                let ingredient = possibilities.iter().next().unwrap().clone();
                unknown_allergens.remove(allergen);
                for f in analyzed_foods.iter_mut() {
                    f.ingredients.remove(&ingredient);
                }
                known_allergens.insert(allergen.clone(), ingredient);
                found = true;
            }
        }
        if !found { break }
    
    }
    let p1 = analyzed_foods.iter().map(|f| f.ingredients.len()).sum::<usize>();
    let mut k: Vec<(&String,&String)> = known_allergens.iter().collect();
    k.sort_by(|a,b| a.0.cmp(b.0));
    let ingredients_with_allergens: Vec<String> = k.iter().map(|(_,i)| i.clone()).cloned().collect();
    let p2 = ingredients_with_allergens[..].join(",");
    
    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}