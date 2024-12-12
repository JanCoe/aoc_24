use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn get_puzzle_map(data: &str) -> HashMap<(isize, isize), char> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((row as isize, col as isize), char))
        })
        .collect()
}

pub fn calc(data: &str) -> usize {
    let puzzle_map = get_puzzle_map(data);

    let antenna_types: HashSet<_> = puzzle_map.values().cloned().filter(|&x| x != '.').collect();

    let mut antinodes = HashSet::new();
    for antenna_type in antenna_types {
        let positions: Vec<_> = puzzle_map
            .iter()
            .filter(|(_, ch)| **ch == antenna_type)
            .map(|(k, _)| k)
            .collect();

        positions
            .iter()
            .cartesian_product(positions.clone())
            .filter(|(&pos1, pos2)| pos1 != *pos2)
            .for_each(|((row1, col1), (row2, col2))| {
                let row = 2 * row1 - row2;
                let col = 2 * col1 - col2;
                if (0 <= row) && (row <= 49) && (0 <= col) && (col <= 49) {
                    antinodes.insert((2 * row1 - row2, 2 * col1 - col2));
                }
            });
    }
    antinodes.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() -> () {
        let data: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14, calc(data));
    }
}
