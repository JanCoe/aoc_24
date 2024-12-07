use crate::part1::create_grid;

pub fn word_search(data: &str) -> i32 {
    let mut total = 0;

    let grid = create_grid(data);

    for ((row, col), &ch) in &grid {
        if ch != 'A' {
            continue;
        }

        let up_left = *grid.get(&(row - 1, col - 1)).unwrap_or(&'E');
        let dn_right = *grid.get(&(row + 1, col + 1)).unwrap_or(&'E');
        let up_right = *grid.get(&(row - 1, col + 1)).unwrap_or(&'E');
        let dn_left = *grid.get(&(row + 1, col - 1)).unwrap_or(&'E');


        if let ('M', 'S') | ('S', 'M') = (up_left, dn_right) {
            if let ('M', 'S') | ('S', 'M') = (dn_left, up_right) {
                total += 1;
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
        assert_eq!(9, word_search(data));
    }
}
