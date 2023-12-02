fn get_calibration(line: &str) -> i32 {
    let n: usize = line.len();
    let c: Vec<char> = line.chars().collect();

    let mut frwd: usize = 0;

    while frwd < n {
        if c[frwd].is_digit(10) {
            break;
        }
        frwd += 1;
    }

    let mut bckwd: usize = n - 1;
    while frwd <= bckwd {
        if c[bckwd].is_digit(10) {
            break;
        }
        bckwd -= 1;
    }
    c[frwd].to_digit(10).unwrap() as i32 * 10 + c[bckwd].to_digit(10).unwrap() as i32
}

pub fn solve(input: &str) -> i32 {
    let mut total_calibration: i32 = 0;
    for line in input.lines() {
        total_calibration += get_calibration(line);
    }
    total_calibration
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::solve;

    const CASE_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn part_a() {
        assert_eq!(solve(CASE_A), 142.into());
    }
}
