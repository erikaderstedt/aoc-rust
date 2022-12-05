use crate::common::Solution;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata_entries: Vec<usize>,

}

impl Node {
    
    fn from<I>(vals: &mut I) -> Node
    where
        I: Iterator<Item = usize> {
            let num_children = vals.next().unwrap();
            let num_metadata_entries = vals.next().unwrap();
            let mut children = vec![];
            let mut metadata_entries = vec![];

            for _ in 0..num_children {
                children.push(Node::from(vals));
            }
            for _ in 0..num_metadata_entries {
                metadata_entries.push(vals.next().unwrap());
            }

            Node { children, metadata_entries}
        }

    fn total_sum_of_metadata_entries(&self) -> usize {
        self.children.iter().map(|c| c.total_sum_of_metadata_entries()).sum::<usize>() 
        + self.metadata_entries.iter().sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.metadata_entries.iter().sum()
        } else {
            self.metadata_entries.iter().map(|index| {
                if *index <= self.children.len() { self.children[*index - 1].value() } else { 0 }        
            }).sum()
        }
    }
}

pub fn solve(input: &str) -> Solution {
    let nodes: Vec<Node> = input.lines()
        .map(|line| {
            Node::from(&mut line.split(" ").map(|s| s.parse::<usize>().unwrap()))
        })
        .collect();

    let p1: usize = nodes.iter().map(|node| node.total_sum_of_metadata_entries()).sum();
    let p2: usize = nodes.iter().map(|node| node.value()).sum();

    Solution::new(p1,p2)
}
