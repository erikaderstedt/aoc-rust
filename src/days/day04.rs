use crate::common::Solution;
use std::collections::BTreeMap;

type PassportInformation<'a> = BTreeMap<&'a str,&'a str>;

pub fn solve(input: &str) -> Solution {
    let passports: Vec<PassportInformation> = input.split("\n\n")
        .map(|pwd_fields| pwd_fields
            .split_whitespace()
            .map(|s: &str| -> (&str, &str) {
                let i = s.find(':').unwrap();
                (&s[..i], &s[(i+1)..])
            })
            .collect())
        .collect();

    let required_fields = ["hcl","ecl","iyr","byr","pid","hgt","eyr"];
    let passports_with_all_required_fields: Vec<PassportInformation> = passports.into_iter()
        .filter(|pwd| required_fields.iter().all(|c| pwd.contains_key(c)))
        .collect();

    let number_of_valid_passports = passports_with_all_required_fields.iter()
        .filter(|passport_info| {
            let h_spec = passport_info["hgt"];
            let mut hcl = passport_info["hcl"].bytes();

            passport_info["hcl"].len() == 7 && 
                hcl.next() == Some(b'#') && 
                hcl.all(|c| c.is_ascii_hexdigit()) &&

            matches!(passport_info["byr"].parse::<i64>(), Ok(b) if 1920 <= b && b <= 2002) &&
            matches!(passport_info["iyr"].parse::<i64>(), Ok(i) if 2010 <= i && i <= 2020) &&
            matches!(passport_info["eyr"].parse::<i64>(), Ok(e) if 2020 <= e && e <= 2030) &&

            passport_info["pid"].len() == 9 &&
            passport_info["pid"].bytes().all(|c| c.is_ascii_digit()) &&

            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport_info["ecl"]) &&

            ((h_spec.ends_with("in") && 
                matches!(h_spec[0..(h_spec.len()-2)].parse::<u32>(), Ok(h) if h >= 59 && h <= 76)) || 
             (h_spec.ends_with("cm") && 
                matches!(h_spec[0..(h_spec.len()-2)].parse::<u32>(), Ok(h) if h >= 150 && h <= 193))) 
        })
        .count();

    Solution { 
        part_1: passports_with_all_required_fields.len().to_string(), 
        part_2: number_of_valid_passports.to_string() 
    }
}