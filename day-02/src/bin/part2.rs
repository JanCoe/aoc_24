use itertools::Itertools;

pub fn safe_reports(data: &str) -> i32 {
    let mut count = 0;

    for report in data.lines() {
        let report: Vec<i32> = report
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        //Case where nothing is removed.
        if crate::part1::check_report(&report) {
            count += 1;
        } else {
            //Case where one is removed.
            let new_length = report.len() - 1;

            for combination in report.iter().combinations(new_length) {
                let combo = combination.into_iter().cloned().collect();
                if crate::part1::check_report(&combo) {
                    count += 1;
                    break;
                }
            }
        };
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_report() -> () {
        let data: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, safe_reports(data));
    }
}
