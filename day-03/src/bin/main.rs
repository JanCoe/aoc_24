mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-03.txt");

    let ans = part1::multiply(data);
    println!("Part 1: {ans}");

    let ans = part2::multiply(data);
    println!("Part 2: {ans}");
}
