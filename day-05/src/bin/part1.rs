#[derive(Debug, Eq, PartialEq)]
enum State {
    Found0,
    Found1,
    Found2,
    Found12,
    Found21,
}

fn check_rule(update: &Vec<u16>, rule: &(u16, u16)) -> bool {
    // 0 is inconclusive, -1 rule is not satisfied, 1 rule is satisfied
    update
        .iter()
        .scan(State::Found0, |state, &x| {
            if *state == State::Found0 && x == rule.0 {
                *state = State::Found1;
                return Some(0);
            }
            if *state == State::Found0 && x == rule.1 {
                *state = State::Found2;
                return Some(0);
            }

            if *state == State::Found1 && x == rule.1 {
                *state = State::Found12;
                return Some(1);
            }

            if *state == State::Found2 && x == rule.0 {
                *state = State::Found21;
                return Some(-1);
            }

            if (*state == State::Found21) | (*state == State::Found12) {
                return None;
            }
            Some(0)
        })
        .min()
        .unwrap()
        != -1
}

pub fn calc(data: &str) -> u16 {
    let rules: Vec<(u16, u16)> = data
        .lines()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let mut pair = line.split('|');
            (
                pair.next().unwrap().parse().unwrap(),
                pair.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let updates: Vec<Vec<u16>> = data
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    updates
        .iter()
        .filter(|x| rules.iter().all(|rule| check_rule(x, &rule)))
        .map(|x| x[(x.len() + 1) / 2 - 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() -> () {
        let data: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, calc(data));
    }
}
