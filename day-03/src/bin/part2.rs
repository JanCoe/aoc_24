use regex::Regex;

#[derive(Eq, PartialEq)]
enum State {
    ENABLED,
    DISABLED,
}

pub fn multiply(data: &str) -> i32 {
    let mut total = 0;
    let mut state = State::ENABLED;

    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

    for cap in re.captures_iter(data) {
        match &cap[0] {
            s if s.starts_with("mul") => {
                if state == State::ENABLED {
                    let first = &cap[1].parse::<i32>().unwrap();
                    let second = &cap[2].parse::<i32>().unwrap();
                    total += first * second;
                }
            }
            "don't()" => state = State::DISABLED,
            "do()" => state = State::ENABLED,
            _ => (),
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> () {
        let data: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, multiply(data));
    }
}
