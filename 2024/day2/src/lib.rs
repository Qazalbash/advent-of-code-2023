mod input;

pub mod part1 {
    pub fn solver(reports: Vec<Vec<i32>>) -> u16 {
        reports
            .iter()
            .map(|report| {
                let n: usize = report.len();
                let is_positive: bool = report[n - 1] > report[0];
                report.windows(2).all(move |w| {
                    let diff = w[1] - w[0];
                    !(0 == diff.abs() || diff.abs() > 3 || (is_positive ^ (diff > 0)))
                })
            })
            .map(|x| match x {
                true => 1_u16,
                false => 0_u16,
            })
            .sum()
    }
}
pub mod part2 {
    fn solve_a_report(report: &[i32]) -> u16 {
        let n: usize = report.len();
        let is_positive: bool = report[n - 1] > report[0];
        for i in 0..(n - 1) {
            let diff = report[i + 1] - report[i];
            if 0 == diff.abs() || diff.abs() > 3 || (is_positive ^ (diff > 0)) {
                // remove ith index and try again
                let mut new_report = report.to_vec();
                new_report.remove(i);
                let mut ith_flag = true;
                for j in 0..(n - 2) {
                    let diff = new_report[j + 1] - new_report[j];
                    if 0 == diff.abs() || diff.abs() > 3 || (is_positive ^ (diff > 0)) {
                        ith_flag = false;
                        break;
                    }
                }
                if !ith_flag {
                    // remove ith+1 index and try again
                    new_report = report.to_vec();
                    new_report.remove(i + 1);
                    for j in 0..(n - 2) {
                        let diff = new_report[j + 1] - new_report[j];
                        if 0 == diff.abs() || diff.abs() > 3 || (is_positive ^ (diff > 0)) {
                            return 0_u16;
                        }
                    }
                }
                return 1_u16;
            }
        }
        1_u16
    }

    pub fn solver(reports: Vec<Vec<i32>>) -> u16 {
        let mut count: u16 = 0;
        for report in reports.iter() {
            count += solve_a_report(report);
        }
        count
    }
}

#[cfg(test)]
mod tests {

    use super::input;
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        let reports = input::input();
        let ans = part1::solver(reports);
        assert_eq!(ans, 524);
    }

    #[test]
    fn test2() {
        let reports = input::input();
        let ans = part2::solver(reports);
        assert_eq!(ans, 569);
    }
}
