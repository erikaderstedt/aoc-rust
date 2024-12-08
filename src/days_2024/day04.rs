// https://adventofcode.com/2024/day/4

use crate::{common::Solution, grid::Grid};

const X: u8 = 'X' as u8;
const M: u8 = 'M' as u8;
const A: u8 = 'A' as u8;
const S: u8 = 'S' as u8;

pub fn solve(input: &str) -> Solution {
    let crossword: Grid<u8> = Grid::load(input)
        .enclosed(' ' as u8)
        .enclosed('.' as u8)
        .enclosed('#' as u8);
    let w = crossword.cols;

    let check_for_xmas = |i2: usize, i3: usize, i4: usize| -> bool {
        crossword.locations[i2] == M && crossword.locations[i3] == A && crossword.locations[i4] == S
    };
    let check_for_mas_x = |i2: usize, i3: usize| -> bool {
        (crossword.locations[i2] == M && crossword.locations[i3] == S)
            || (crossword.locations[i2] == S && crossword.locations[i3] == M)
    };

    let p1 = crossword
        .positions()
        .filter(|p| crossword.get(p) == Some(X))
        .map(|p| -> Vec<bool> {
            vec![
                check_for_xmas(
                    p.row * w + p.column + 1,
                    p.row * w + p.column + 2,
                    p.row * w + p.column + 3,
                ),
                check_for_xmas(
                    p.row * w + p.column - 1,
                    p.row * w + p.column - 2,
                    p.row * w + p.column - 3,
                ),
                check_for_xmas(
                    (p.row + 1) * w + p.column,
                    (p.row + 2) * w + p.column,
                    (p.row + 3) * w + p.column,
                ),
                check_for_xmas(
                    (p.row - 1) * w + p.column,
                    (p.row - 2) * w + p.column,
                    (p.row - 3) * w + p.column,
                ),
                check_for_xmas(
                    (p.row + 1) * w + p.column + 1,
                    (p.row + 2) * w + p.column + 2,
                    (p.row + 3) * w + p.column + 3,
                ),
                check_for_xmas(
                    (p.row - 1) * w + p.column + 1,
                    (p.row - 2) * w + p.column + 2,
                    (p.row - 3) * w + p.column + 3,
                ),
                check_for_xmas(
                    (p.row + 1) * w + p.column - 1,
                    (p.row + 2) * w + p.column - 2,
                    (p.row + 3) * w + p.column - 3,
                ),
                check_for_xmas(
                    (p.row - 1) * w + p.column - 1,
                    (p.row - 2) * w + p.column - 2,
                    (p.row - 3) * w + p.column - 3,
                ),
            ]
        })
        .flatten()
        .filter(|x| *x)
        .count();

    let p2 = crossword
        .positions()
        .filter(|p| crossword.get(p) == Some(A))
        .map(|p| -> bool {
            check_for_mas_x(
                (p.row - 1) * w + p.column - 1,
                (p.row + 1) * w + p.column + 1,
            ) && check_for_mas_x(
                (p.row + 1) * w + p.column - 1,
                (p.row - 1) * w + p.column + 1,
            )
        })
        .filter(|x| *x)
        .count();

    Solution::new(p1, p2)
}
