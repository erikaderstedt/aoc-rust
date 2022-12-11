use std::fs::File;
use std::io::Read;
use std::num;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub struct Solution {
    pub part_1: String,
    pub part_2: String,
}

impl Solution {
    pub fn new(p1: impl ToString, p2: impl ToString) -> Solution {
        Solution { part_1: p1.to_string(), part_2: p2.to_string() }
    }
}

pub type Solver = fn(&str) -> crate::common::Solution;

pub fn day_input_filename(year: &str, day_s: &str) -> PathBuf {
    let day = day_s.parse::<u8>().unwrap();
    let padded_day = format!("{:02}", day);
    Path::new("inputs").join(year).join(format!("day{}.in", padded_day))
}

pub fn get_file_contents(path: &Path) -> std::io::Result<String> {
    if path == Path::new("-") {
        read_all(std::io::stdin())
    } else if !path.exists() {
            Ok("".to_string())
    } else {
        read_all(
            File::open(&path)
                .unwrap_or_else(|_| panic!("Input file not found: {:?}", path)),
        )
    }
}

fn read_all<I: Read>(mut source: I) -> std::io::Result<String> {
    let mut contents: String = String::new();
    source.read_to_string(&mut contents).map(|_u| contents)
}

pub fn parsed_from_each_line<T: FromStr>(input: &str) -> Vec<T> 
where T::Err: std::fmt::Display
{
    input.lines().map(|x| 
        match x.parse::<T>() {
            Ok(v) => v,
            Err(e) => panic!("Bad input on line '{}': {}", x, e),
        }
    ).collect()
}

pub fn read_5x6_characters_off_grid(grid: &Vec<bool>) -> Result<String, &str> {
    if grid.len() % 30 != 0 {
        return Err("Grid size must be a multiple of 30.")
    }
    let num_chars = grid.len() / 30;
    let width = grid.len() / 6;
    Ok((0..num_chars)
        .map(|j| {
            let mut v: u32 = 0;
            let mut i = 0;
            for r in 0..6 {
                for c in 0..5 {
                    if grid[r*width + j*5 + c] { v += 1 << i; }
                    i += 1;
                }
            }
            match v {
                0x1297A526 => 'A',
                0x0E949D27 => 'B',
                0x0C908526 => 'C',
                0x1E109C2F => 'E',
                0x02109C2F => 'F',
                0x1C968526 => 'G',
                0x1294BD29 => 'H',
                0x1C42108E => 'I',
                0x0C94210C => 'J',
                0x12528CA9 => 'K',
                0x1E108421 => 'L',
                0x0C94A526 => 'O',
                0x0213A527 => 'P',
                0x1253A527 => 'R',
                0x0E83042E => 'S',
                0x0C94A529 => 'U',
                0x08422A31 => 'Y',
                0x1E11110F => 'Z',
                v => {
                    println!("Unrecognized character, value 0x{:08X}", v);
                    for y in 0..6 {
                        for x in 0..5 {
                            if !grid[y*5 + x] {
                                print!(" ");
                            } else {
                                print!("█");
                            }
                        }
                        println!("");
                    }
                    '?'
                },
            }
        })
        .collect())
}
