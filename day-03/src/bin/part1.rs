use regex::Regex;

pub fn multiply(data: &str) -> i32 {
    let mut total = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(data) {
        let first = &cap[1].parse::<i32>().unwrap();
        let second = &cap[2].parse::<i32>().unwrap();
        total += first * second;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> () {
        let data: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, multiply(data));
    }
}
