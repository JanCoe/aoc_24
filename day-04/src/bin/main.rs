mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-04.txt");

    let ans = part1::word_search(data);
    println!("Part 1: {ans}");

    let ans = part2::word_search(data);
    println!("Part 2: {ans}");
}
