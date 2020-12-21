use crate::common::Solution;
use std::collections::HashSet;

struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Food {
        let i = s.find('(').unwrap();
        let ingredients: HashSet<&'a str> = s[..(i-1)].split(' ').collect();
        let allergens: HashSet<&'a str> = s[(i+10)..(s.len()-1)].split(", ").collect();
        Food { ingredients, allergens }
    }
}

pub fn solve(input: &str) -> Solution {
    let foods: Vec<Food> = input.lines().map(|s| Food::parse(s)).collect();
    let all_allergens: HashSet<&str> = foods.iter().map(|f| f.allergens.iter()).flatten().cloned().collect();

    let mut known_allergens: Vec<(&str,&str)> = Vec::new();
    let mut unknown_allergens = all_allergens.clone();
    let mut matched_ingredients: HashSet<&str> = HashSet::new();
    
    while unknown_allergens.len() > 0 {
        unknown_allergens = unknown_allergens.clone().into_iter().filter(|u| {
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
                known_allergens.push((u, candidate_ingredients[0]));
                unknown_allergens.remove(u);
                matched_ingredients.insert(candidate_ingredients[0]);
                false
            } else {
                true
            }
        })
        
        .collect();
    }
    let p1 = foods.iter()
        .map(|food| food.ingredients
            .difference(&matched_ingredients)
            .count())
        .sum::<usize>();
        
    known_allergens.sort_by(|a,b| a.0.cmp(&b.0));
    let dangerous_ingredients:Vec<&str> = known_allergens.into_iter().map(|(_,i)| i).collect();
    let p2 = dangerous_ingredients[..].join(",");

    Solution { part_1: p1.to_string(), part_2: p2.to_string() }
}