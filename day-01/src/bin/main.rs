mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-01.txt");

    let ans = part1::calc_distance(data);
    println!("Part 1: {ans}");

    let ans = part2::calc_score(data);
    println!("Part 2: {ans}");
}
