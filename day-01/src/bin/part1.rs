pub fn calc_distance(data: &str) -> i32 {
    let mut left = vec![];
    let mut right = vec![];

    for pairs in data.lines() {
        let mut pair = pairs.split_whitespace();
        left.push(pair.next().unwrap().parse::<i32>().unwrap());
        right.push(pair.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    left.into_iter().zip(right).map(|(l, r)| (l - r).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance() -> () {
        let data: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, calc_distance(data));
    }
}
