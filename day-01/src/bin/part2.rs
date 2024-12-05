pub fn calc_score(data: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for pairs in data.lines() {
        let mut pair = pairs.split_whitespace();
        left.push(pair.next().unwrap().parse::<usize>().unwrap());
        right.push(pair.next().unwrap().parse::<usize>().unwrap());
    }

    left.into_iter()
        .map(|l| right.iter().filter(|&r| l == *r).count() * l)
        .sum()
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
        assert_eq!(31, calc_score(data));
    }
}
