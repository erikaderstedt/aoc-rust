// https://adventofcode.com/2022/day/20

use itertools::Itertools;

use crate::common::Solution;
use std::collections::VecDeque;

type List = VecDeque<(usize,i64)>;

struct ListMixer {
    ordering: List,
    mixed: List
}

impl ListMixer {
    fn new(list: List) -> ListMixer { ListMixer { ordering: list.clone(), mixed: list }}
    
    fn mix(&mut self) {    
        for (label, num) in self.ordering.iter() {
            let index: usize = self.mixed.iter().find_position(|j| j.0 == *label).unwrap().0;
            let value = self.mixed.remove(index).unwrap();
            let new_index = (((index as i64) + *num + (2811589153)*(self.mixed.len() as i64)) as usize) % self.mixed.len();
            self.mixed.insert(new_index, value);
        }
    }

    fn grove_coordinates(&self) -> i64 {
        let i0 = self.mixed.iter().find_position(|&j| j.1 == 0i64).unwrap().0;
        let n1 = self.mixed[(i0 + 1000) % self.mixed.len()].1;
        let n2 =  self.mixed[(i0 + 2000) % self.mixed.len()].1;
        let n3 =  self.mixed[(i0 + 3000) % self.mixed.len()].1;
        n1+n2+n3
    }
}

pub fn solve(input: &str) -> Solution {
    let list: List = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .enumerate()
        .collect();
    let decrypted: List = list
        .iter()
        .map(|(i, n)| (i.clone(), n * 811589153))
        .collect();

    let p1 = {
        let mut mixer = ListMixer::new(list);
        mixer.mix();
        mixer.grove_coordinates()
    };

    let p2 = {
        let mut mixer = ListMixer::new(decrypted);
        for _ in 0..10 {
            mixer.mix();            
        }
        mixer.grove_coordinates()
    };

    Solution::new(p1,p2)
}
