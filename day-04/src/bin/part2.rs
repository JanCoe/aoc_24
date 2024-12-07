use crate::part1::create_grid;

pub fn word_search(data: &str) -> usize {
    let grid = create_grid(data);

    grid.iter()
        .filter(|(_, &ch)| ch == 'A')
        .filter(|((row, col), _)| {
            let up_l = *grid.get(&(row - 1, col - 1)).unwrap_or(&'E');
            let up_r = *grid.get(&(row - 1, col + 1)).unwrap_or(&'E');
            let dn_l = *grid.get(&(row + 1, col - 1)).unwrap_or(&'E');
            let dn_r = *grid.get(&(row + 1, col + 1)).unwrap_or(&'E');

            matches!((up_l, dn_r), ('M', 'S') | ('S', 'M'))
                && matches!((dn_l, up_r), ('M', 'S') | ('S', 'M'))
        })
        .count()
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
