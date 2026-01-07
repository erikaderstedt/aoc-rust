// https://adventofcode.com/2015/day/15

use crate::common::Solution;
use itertools::Itertools;
use rand::Rng;
use regex::Regex;

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

const TOTAL: i64 = 100;
const TARGET_CALORIES: i64 = 500;

fn gradient_search_for_best_recipe(ingredients: &[Ingredient; 4]) -> i64 {
    let mut rng = rand::rng();

    // Find any valid solution at random.
    let mut values = [TOTAL / 4; 4];
    while result(&values, &ingredients) == 0 {
        values[0] = rng.random_range(0..TOTAL);
        values[1] = rng.random_range(0..(TOTAL - values[0]));
        values[2] = rng.random_range(0..(TOTAL - values[0] - values[1]));
        values[3] = TOTAL - values[0] - values[1] - values[2];
    }

    let mut best = result(&values, &ingredients);

    let permutations = [
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 2),
        (1, 3),
        (2, 0),
        (2, 1),
        (2, 3),
        (3, 0),
        (3, 1),
        (3, 2),
    ];

    loop {
        // Which pair (x+1, y-1) gives the best improvement
        let (i, j) = permutations
            .iter()
            .filter(|(i, j)| values[*i as usize] < 100 && values[*j as usize] > 0)
            .max_by_key(|(i, j)| {
                let mut v = values.clone();
                v[*i as usize] += 1;
                v[*j as usize] -= 1;
                result(&v, &ingredients)
            })
            .unwrap();
        values[*i as usize] += 1;
        values[*j as usize] -= 1;
        let r = result(&values, &ingredients);
        if r < best {
            // No more improvement. We are at a local maximum,
            // and there should be only one maximum given the problem
            // formulation.
            break best;
        } else {
            best = r;
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let r = Regex::new(
        "^\\w+: capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (-?\\d+)"
    )
    .unwrap();

    let ingredients: [Ingredient; 4] = input
        .lines()
        .map(|line| {
            let (_, [capacity, durability, flavor, texture, calories]) =
                r.captures(line).unwrap().extract();
            Ingredient {
                capacity: capacity.parse::<i64>().unwrap(),
                durability: durability.parse::<i64>().unwrap(),
                flavor: flavor.parse::<i64>().unwrap(),
                texture: texture.parse::<i64>().unwrap(),
                calories: calories.parse::<i64>().unwrap(),
            }
        })
        .collect_array()
        .unwrap();

    let p1 = gradient_search_for_best_recipe(&ingredients);

    // For part two a gradient search is impractical because it is difficult
    // to find the next valid item. Instead, we have an additional constraint
    // on the solution reducing the number of free variables from 3 to 2.
    // This lets us solve it easily by Gaussian elimination

    // Find the two ingredients with the largest coefficients for the calories.
    // This will reduce the search space significantly.
    let (a, b, c, d) = (0..4)
        .sorted_by_key(|i| ingredients[*i].calories)
        .collect_tuple()
        .unwrap();

    let coeff_a = ingredients[a].calories;
    let coeff_b = ingredients[b].calories;
    let coeff_c = ingredients[c].calories;
    let coeff_d = ingredients[d].calories;

    let mut p2 = 0;
    for num_c in 0..=(TARGET_CALORIES / coeff_c) {
        for num_d in 0..=(TARGET_CALORIES / coeff_d) {
            let remaining_calories = TARGET_CALORIES - num_c * coeff_c - num_d * coeff_d;
            let remaining_tbsp = TOTAL - num_c - num_d;
            if remaining_calories < 0 || remaining_tbsp < 0 {
                continue;
            }
            // Two remaining constraints, we can solve for num_a and num_b
            // coeff_a * num_a + coeff_b * num_b = remaining_calories
            //           num_a +           num_b = remaining_tbsp;
            let m = remaining_calories - remaining_tbsp * coeff_a;
            let n = coeff_b - coeff_a;
            if m % n != 0 {
                continue;
            }
            let num_b = m / n;
            let num_a = remaining_tbsp - num_b;
            if num_a < 0 || num_b < 0 {
                continue;
            }
            let mut all = [0i64; 4];
            all[a] = num_a;
            all[b] = num_b;
            all[c] = num_c;
            all[d] = num_d;

            let r = result(&all, &ingredients);
            if r > p2 {
                p2 = r;
            }
        }
    }

    Solution::new(p1, p2)
}

fn result(v: &[i64; 4], ingredients: &[Ingredient; 4]) -> i64 {
    let capacity = ingredients
        .iter()
        .zip(v.iter())
        .map(|(x, y)| x.capacity * *y)
        .sum::<i64>();
    let durability = ingredients
        .iter()
        .zip(v.iter())
        .map(|(x, y)| x.durability * *y)
        .sum::<i64>();
    let flavor = ingredients
        .iter()
        .zip(v.iter())
        .map(|(x, y)| x.flavor * *y)
        .sum::<i64>();
    let texture = ingredients
        .iter()
        .zip(v.iter())
        .map(|(x, y)| x.texture * *y)
        .sum::<i64>();
    if capacity > 0 && durability > 0 && flavor > 0 && texture > 0 {
        capacity * texture * flavor * durability
    } else {
        0
    }
}
