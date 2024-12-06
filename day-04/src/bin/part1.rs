use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

pub fn word_search(data: &str) -> i32 {
    let mut total = 0;

    let grid: BTreeMap<(isize, isize), char> = data
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
        .collect();

    let values: RangeInclusive<isize> = -1..=1;

    let directions = values.clone().into_iter().cartesian_product(values);

    for ((row, col), &ch) in &grid {
        if ch == 'X' {
            for (row_inc, col_inc) in directions.clone() {
                let next = grid.get(&(row+row_inc, col+col_inc)).unwrap_or(&'E');
                if next == &'M' {
                    let next = grid.get(&(row + 2*row_inc, col + 2*col_inc)).unwrap_or(&'E');
                    if next == &'A' {
                        let next = grid.get(&(row + 3*row_inc, col + 3*col_inc)).unwrap_or(&'E');
                        if next == &'S' {
                            total += 1;
                        }
                    }

                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search() -> () {
        let data: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, word_search(data));
    }
}
