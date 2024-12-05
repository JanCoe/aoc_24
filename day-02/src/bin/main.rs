mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-02.txt");

    let ans = part1::safe_reports(data);
    println!("Part 1: {ans}");

    let ans = part2::safe_reports(data);
    println!("Part 2: {ans}");
}
