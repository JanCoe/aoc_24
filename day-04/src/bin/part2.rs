use std::collections::BTreeMap;

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

    for ((row, col), &ch) in &grid {
        if ch != 'A' {
            continue;
        }

        let up_left = *grid.get(&(row - 1, col - 1)).unwrap_or(&'E');
        let dn_right = *grid.get(&(row + 1, col + 1)).unwrap_or(&'E');
        let up_right = *grid.get(&(row - 1, col + 1)).unwrap_or(&'E');
        let dn_left = *grid.get(&(row + 1, col - 1)).unwrap_or(&'E');

        match (up_left, dn_right) {
            ('M', 'S') => (),
            ('S', 'M') => (),
            _ => continue,
        }

        match (dn_left, up_right) {
            ('M', 'S') => (),
            ('S', 'M') => (),
            _ => continue,
        }

        total += 1;
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
        assert_eq!(9, word_search(data));
    }
}
