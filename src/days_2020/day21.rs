use crate::common::Solution;
use itertools::Itertools;
use std::collections::{HashSet,BTreeMap};

struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

pub fn solve(input: &str) -> Solution {
    let mut foods: Vec<Food> = input.lines().map(|s| -> Food {
        let i = s.find('(').unwrap();
        Food { 
            ingredients: s[..(i-1)].split(' ').collect(), 
            allergens: s[(i+10)..(s.len()-1)].split(", ").collect() 
        }
    }).collect();

    let mut unknown_allergens: HashSet<&str> = foods.iter().map(|f| f.allergens.iter()).flatten().cloned().collect();
    let mut known_allergens: BTreeMap<&str,&str> = BTreeMap::new();
    
    while unknown_allergens.len() > 0 {
        let j = unknown_allergens.len();
        unknown_allergens = unknown_allergens.into_iter().filter(|u| {
            let foods_with_this_allergen: Vec<&Food> = foods.iter().filter(|f| f.allergens.contains(u)).collect();
            let candidate_ingredients:HashSet<&str> = foods_with_this_allergen.iter()
                .skip(1)
                .fold(foods_with_this_allergen[0].ingredients.clone(),|i,f| i.intersection(&f.ingredients).cloned().collect());
            
            if candidate_ingredients.len() == 1 {
                let ingredient = candidate_ingredients.into_iter().next().unwrap();
                for f in foods.iter_mut() { f.ingredients.remove(ingredient); }
                known_allergens.insert(u, ingredient);
                false
            } else {
                true
            }
        }).collect();
        assert!(j != unknown_allergens.len());
    }

    let p1 = foods.iter().map(|food| food.ingredients.len()).sum::<usize>();
    let p2 = known_allergens.values().join(",");

    Solution::new(p1,p2)
}