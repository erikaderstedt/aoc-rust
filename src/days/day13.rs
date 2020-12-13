use crate::common::Solution;

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn extended_euclidean_algorithm(a: i64, b: i64) -> (i64,i64,i64) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    loop {
        let q = r0 / r1;
        let r = r0 - q * r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;
        if r == 0 {
            assert!(a*s1 + b*t1 == r1);
            return (r1, s1, t1)            
        }
        t0 = t1; t1 = t;
        s0 = s1; s1 = s;
        r0 = r1; r1 = r;
    }
}

// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
fn modular_inverse(a: i64, b: i64) -> Option<i64> {
    let (remainder, bezout_1, _bezout_2) = extended_euclidean_algorithm(a,b);
    match remainder {
        1 => Some((bezout_1 % b + b) % b),
        _ => None, 
    }
}

pub fn solve(input: &str) -> Solution {

    let earliest = input.lines().take(1).next().unwrap().parse::<i64>().expect("");
    let buses: Vec<(i64,i64)> = input.lines().skip(1).take(1).next().unwrap().split(',')
        .enumerate()
        .filter(|(_,v)| *v != "x")
        .map(|(i,c)| {      
            let v = c.parse::<i64>().unwrap();
            (v - (i as i64), v)
        })
        .collect();

    let p1 = buses.iter().map(|&b| {
        let missed_by =  earliest % b.1;
        let minutes_left = b.1 - missed_by;
        (minutes_left, b.1)
    }).min_by(|a,b| a.0.cmp(&b.0)).unwrap();

    let total_product = buses.iter().fold(1, |a,(_,n)| a*n);
    let p2 = buses.iter().fold(0, |a, (r, n)| {
        let n_i = total_product / *n;
        a + r * modular_inverse(n_i, *n).unwrap() * n_i
    }) % total_product;

    Solution { part_1: (p1.0 * p1.1).to_string(), part_2: p2.to_string() }
}
