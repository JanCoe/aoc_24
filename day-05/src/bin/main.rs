mod part1;
mod part2;

fn main() {
    let data = include_str!("../../../input/day-05.txt");

    let ans = part1::func(data);
    println!("Part 1: {ans}");

    // let ans = part2::func(data);
    // println!("Part 2: {ans}");
}
