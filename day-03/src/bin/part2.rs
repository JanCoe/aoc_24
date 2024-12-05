use regex::Regex;

#[derive(Debug)]
enum State {
    ENABLED,
    DISABLED,
}

pub fn multiply(data: &str) -> i32 {
    let mut total = 0;

    let re_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let re_dont= Regex::new(r"don't\(\)").unwrap();
    let re_do= Regex::new(r"do\(\)").unwrap();

    let combined = r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)";
    let combo = Regex::new(combined).unwrap();

    let patterns: Vec<_> = combo.captures_iter(data).collect();
    println!("{:?}", patterns);

    let mut state = State::ENABLED;

    for cap in combo.captures_iter(data) {
        println!("start state {:?}", state);
        println!("start total {:?}", total);
        println!("cap {:?}", &cap[0]);

        if re_mul.is_match(&cap[0]) {
            if let State::ENABLED = state {
                let first = &cap[1].parse::<i32>().unwrap();
                let second = &cap[2].parse::<i32>().unwrap();
                total += first * second;
            }
        }

        if re_dont.is_match(&cap[0]) {
            state = State::DISABLED;
            println!("State disabled");
        };

        if re_do.is_match(&cap[0]) {
            state = State::ENABLED;
            println!("State enabled");
        };

        println!("end state {:?}", state);
        println!("end total {:?}", total);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> () {
        let data: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, multiply(data));
    }
}
