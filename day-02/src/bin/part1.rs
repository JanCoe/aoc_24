use itertools::Itertools;

pub fn safe_reports(data: &str) -> i32 {
    let mut count = 0;

    for report in data.lines() {
        let report: Vec<i32> = report
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_report(&report) {
            count += 1
        };
    }
    count
}

pub fn check_report(report: &Vec<i32>) -> bool {
    let diff: Vec<i32> = report
        .iter()
        .tuple_windows()
        .map(|(&one, &two)| two - one)
        .collect();

    let min = diff.iter().min().unwrap();
    let max = diff.iter().max().unwrap();

    if ((*min >= 1) & (*max <= 3)) | ((*min >= -3) & (*max <= -1)) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_reports() -> () {
        let data: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, safe_reports(data));
    }
}
