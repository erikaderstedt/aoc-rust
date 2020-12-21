use crate::common::Solution;

#[derive(Clone)]
enum Rule {
    Unimplemented,
    Character(u8),
    Single(Vec<usize>),
    Multiple((Vec<usize>, Vec<usize>)),
}

impl std::str::FromStr for Rule {
    type Err = &'static str;

    fn from_str(ch: &str) -> std::result::Result<Rule,Self::Err> {
        Ok(match ch.find('|') {
            Some(p) => {
                Rule::Multiple((ch[..(p-1)].split(' ').map(|c| c.parse::<usize>().unwrap()).collect(),
                ch[(p+2)..].split(' ').map(|c| c.parse::<usize>().unwrap()).collect()))
            },
            None => if ch.contains("\"") {
                Rule::Character(ch.as_bytes()[1])
            } else {
                Rule::Single(ch.split(' ').map(|c| c.parse::<usize>().unwrap()).collect())
            },
        })
    }
}

struct RulesList { rules: Vec<Rule> }

impl RulesList {

    fn load(input: &str) -> RulesList {
        let mut rules = vec![Rule::Unimplemented; 200];

        for line in input.lines().take_while(|&line| line != "") {
            let c = line.find(':').unwrap();
            let num = line[..c].parse::<usize>().unwrap();
            let rule = line[(c+2)..].parse::<Rule>().unwrap();
            rules[num] = rule;
        }
        RulesList { rules }
    }

    fn match_sequence<'a>(&self, indices: &Vec<usize>, s: &'a [u8], q: &mut Vec<usize>) -> bool {
        q.extend(indices.iter().rev());
        self.match_rules(s, q)
    }

    fn match_rules<'a>(&self, s: &'a [u8], q: &mut Vec<usize>) -> bool {
        match q.pop() {
            Some(rule_index) => match &self.rules[rule_index] {
                Rule::Character(c1) => matches!(s.get(0), Some(c) if c == c1) && self.match_rules(&s[1..], q),
                Rule::Single(indices) => self.match_sequence(&indices, s, q),
                Rule::Multiple((left,right)) => {
                    self.match_sequence(&left, s, &mut q.clone()) ||
                    self.match_sequence(&right, s, &mut q.clone())
                },
                Rule::Unimplemented => { false },
            }
            None => s.len() == 0,
        }
    }

    fn matches_to_rule_0(&self, messages: &Vec<&str>) -> usize {
        messages.iter().filter(|line| self.match_rules(line.as_bytes(), &mut vec![0])).count()
    }
}

pub fn solve(input: &str) -> Solution {
    let mut matcher = RulesList::load(input);

    let messages: Vec<&str> = input.lines()
        .skip_while(|&line| line != "")
        .skip(1).collect();

    let p1 = matcher.matches_to_rule_0(&messages);
    
    matcher.rules[8] = Rule::Multiple((vec![42], vec![42,8]));
    matcher.rules[11] = Rule::Multiple((vec![42,31], vec![42,11,31]));
    
    let p2 = matcher.matches_to_rule_0(&messages);

    Solution::new(p1,p2)
}
