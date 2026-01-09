// https://adventofcode.com/2016/day/7

use crate::common::Solution;

fn has_abba(s: &str) -> bool {
    let s = s.as_bytes();
    (0..(s.len() - 3)).any(|i| s[i] == s[i + 3] && s[i + 1] == s[i + 2] && s[i] != s[i + 1])
}

struct Ipv7<'a> {
    supernets: Vec<&'a str>,
    hypernets: Vec<&'a str>,
}

impl<'a> Ipv7<'a> {
    fn supports_tls(&self) -> bool {
        self.supernets.iter().any(|s| has_abba(s)) && !self.hypernets.iter().any(|h| has_abba(h))
    }

    fn supports_ssl(&self) -> bool {
        self.supernets.iter().any(|s| {
            let s = s.as_bytes();
            (0..(s.len() - 2)).any(|i| {
                s[i] == s[i + 2]
                    && s[i] != s[i + 1]
                    && self.hypernets.iter().any(|h| {
                        let h = h.as_bytes();
                        (0..(h.len() - 2))
                            .any(|j| h[j] == s[i + 1] && h[j + 1] == s[i] && h[j + 2] == s[i + 1])
                    })
            })
        })
    }
}

pub fn solve(input: &str) -> Solution {
    let mut ips = vec![];
    for line in input.lines() {
        let mut supernets: Vec<&str> = vec![];
        let mut hypernets: Vec<&str> = vec![];
        let mut i = 0;

        while let Some(index_of_next_bracket) =
            (i..line.len()).find(|j| line.as_bytes()[*j] == '[' as u8)
        {
            supernets.push(&line[i..index_of_next_bracket]);
            i = index_of_next_bracket + 1;
            let k = (i..line.len())
                .find(|j| line.as_bytes()[*j] == ']' as u8)
                .unwrap();
            hypernets.push(&line[i..k]);
            i = k + 1;
        }
        supernets.push(&line[i..line.len()]);
        ips.push(Ipv7 {
            supernets,
            hypernets,
        });
    }

    let p1 = ips.iter().filter(|ip| ip.supports_tls()).count();
    let p2 = ips.iter().filter(|ip| ip.supports_ssl()).count();

    Solution::new(p1, p2)
}
