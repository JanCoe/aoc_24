mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-03.txt");

    let ans = part1::multiply(data);
    println!("Part 1: {ans}");

    // let data: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let ans = part2::multiply(data);
    println!("Part 2: {ans}");
}