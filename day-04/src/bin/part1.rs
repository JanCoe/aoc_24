use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::RangeInclusive;

pub fn create_grid(data: &str) -> BTreeMap<(isize, isize), char> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| ((row as isize, col as isize), ch))
        })
        .collect()
}

fn get_char(
    grid: &BTreeMap<(isize, isize), char>,
    centre: &(isize, isize),
    direction: &(isize, isize),
    distance: isize,
) -> char {
    *grid
        .get(&(
            centre.0 + direction.0 * distance,
            centre.1 + direction.1 * distance,
        ))
        .unwrap_or(&'E')
}

pub fn word_search(data: &str) -> usize {
    let grid = create_grid(data);

    let values: RangeInclusive<isize> = -1..=1;
    let directions: Vec<(isize, isize)> = values.clone().into_iter().cartesian_product(values).collect();

    grid.iter()
        .filter(|(_, &ch)| ch == 'X')
        .map(|(centre, _)| {
            directions
                .iter()
                .filter(|&direction| {
                    get_char(&grid, centre, direction, 1) == 'M'
                        && get_char(&grid, centre, direction, 2) == 'A'
                        && get_char(&grid, centre, direction, 3) == 'S'
                })
                .count()
        })
        .sum()
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
