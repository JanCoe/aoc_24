mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-08.txt");

    let ans = part1::calc(data, 49, 49);
    println!("Day 8, Part 1: {ans}");

    // let ans = part2::calc(data);
    // println!("Day 8, Part 2: {ans}");
}
