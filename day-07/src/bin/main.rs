mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-07.txt");

    let ans = part1::calc(data);
    println!("Day 7, Part 1: {ans}");

    // let ans = part2::calc(data);
    // println!("Day 7, Part 2: {ans}");
}
