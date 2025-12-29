use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Solution {
    pub part_1: String,
    pub part_2: String,
}

impl Solution {
    pub fn new(p1: impl ToString, p2: impl ToString) -> Solution {
        Solution {
            part_1: p1.to_string(),
            part_2: p2.to_string(),
        }
    }
}

pub type Solver = fn(&str) -> crate::common::Solution;

pub fn day_input_filename(year: &str, day_s: &str) -> PathBuf {
    let day = day_s.parse::<u8>().unwrap();
    let padded_day = format!("{:02}", day);
    Path::new("inputs")
        .join(year)
        .join(format!("day{}.in", padded_day))
}

pub fn get_file_contents(path: &Path) -> std::io::Result<String> {
    if path == Path::new("-") {
        read_all(std::io::stdin())
    } else if !path.exists() {
        Ok("".to_string())
    } else {
        read_all(File::open(&path).unwrap_or_else(|_| panic!("Input file not found: {:?}", path)))
    }
}

fn read_all<I: Read>(mut source: I) -> std::io::Result<String> {
    let mut contents: String = String::new();
    source.read_to_string(&mut contents).map(|_u| contents)
}

#[allow(dead_code)]
pub fn parsed_from_each_line<T: FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Display,
{
    input
        .lines()
        .map(|x| match x.parse::<T>() {
            Ok(v) => v,
            Err(e) => panic!("Bad input on line '{}': {}", x, e),
        })
        .collect()
}
