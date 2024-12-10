use itertools;
use itertools::Itertools;

pub fn calc(data: &str) -> usize {
    let calibrations: Vec<(usize, Vec<usize>)> = data
        .lines()
        .map(|line| {
            let mut split_line = line.split(':');
            let first = split_line.next().unwrap().parse().unwrap();
            let second: Vec<_> = split_line
                .next()
                .unwrap()
                .split_whitespace()
                .into_iter()
                .map(|x| x.parse().unwrap())
                .collect();
            (first, second)
        })
        .collect();

    let mut result = 0;
    for calibration in calibrations {
        let answer = calibration.0;
        let num_operators = calibration.1.iter().len() - 1;
        let first_number = *calibration.1.iter().next().unwrap();
        for operators in (1..=num_operators).map(|_| 0..=1).multi_cartesian_product() {
            let mut calculation = first_number;
            for (operator, number) in operators.into_iter().zip(calibration.1.iter().skip(1)) {
                if operator == 0 {
                    calculation *= number;
                } else {
                    calculation += number;
                }
            }
            if calculation == answer {
                result += calculation;
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() -> () {
        let data: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, calc(data));
    }
}