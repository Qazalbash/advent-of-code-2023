mod input;

pub mod part1 {
    fn solve_a_report(report: &[i32]) -> u16 {
        let n: usize = report.len();
        let is_positive: bool = report[n - 1] > report[0];
        for i in 0..(n - 1) {
            let diff = report[i + 1] - report[i];
            if 0 == diff.abs() || diff.abs() > 3 || (is_positive ^ (diff > 0)) {
                return 0_u16;
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
pub mod part2 {
    /// Incomplete
    pub fn solver(reports: Vec<Vec<i32>>) -> u16 {
        0
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
        assert_eq!(ans, 524);
    }
}
