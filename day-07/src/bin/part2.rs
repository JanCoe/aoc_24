use crate::part1::get_calibrations;
use itertools;
use itertools::Itertools;

pub fn calc(data: &str) -> usize {
    let calibrations = get_calibrations(data);

    let mut result = 0;
    for calibration in calibrations {
        let num_operators = calibration.1.iter().len() - 1;
        let first_number = *calibration.1.iter().next().unwrap();
        for operators in (1..=num_operators).map(|_| 0..=2).multi_cartesian_product() {
            let mut calculation = first_number;
            for (operator, number) in operators.into_iter().zip(calibration.1.iter().skip(1)) {
                match operator {
                    0 => calculation += number,
                    1 => calculation *= number,
                    _ => {
                        calculation = format!("{calculation}{number}").parse().unwrap();
                    }
                }
            }
            if calculation == calibration.0 {
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
        assert_eq!(11387, calc(data));
    }
}
