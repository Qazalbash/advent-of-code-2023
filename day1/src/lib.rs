pub mod part1 {
    fn get_calibration(line: &str) -> i32 {
        let n: usize = line.len();
        let c: Vec<char> = line.chars().collect();

        let mut frwd: usize = 0;
        while frwd < n {
            if c[frwd].is_ascii_digit() {
                break;
            }
            frwd += 1;
        }

        let mut bckwd: usize = n - 1;
        while frwd <= bckwd {
            if c[bckwd].is_ascii_digit() {
                break;
            }
            bckwd -= 1;
        }
        c[frwd].to_digit(10).unwrap() as i32 * 10 + c[bckwd].to_digit(10).unwrap() as i32
    }

    pub fn solve(input: &str) -> i32 {
        let mut total_calibration: i32 = 0;
        input
            .lines()
            .for_each(|line: &str| total_calibration += get_calibration(line));
        total_calibration
    }
}

pub mod part2 {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    const REV_DIGITS: [&str; 9] = [
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    fn get_calibration(line: &str) -> i32 {
        let n: usize = line.len();
        let c: Vec<char> = line.chars().collect();

        let mut frwd: usize = 0;
        let mut bckwd: usize = n - 1;

        let mut fd: Option<i32> = None;
        let mut bd: Option<i32> = None;

        while frwd < n {
            if c[frwd].is_ascii_digit() {
                fd = Some(c[frwd].to_digit(10).unwrap() as i32);
                break;
            }
            frwd += 1;
        }

        while frwd <= bckwd {
            if c[bckwd].is_ascii_digit() {
                bd = Some(c[bckwd].to_digit(10).unwrap() as i32);
                break;
            }
            bckwd -= 1;
        }

        bckwd = n - bckwd;
        let reversed_line = line.chars().rev().collect::<String>();

        for i in 0..9 {
            let frwd_pos: usize = line.find(DIGITS[i]).unwrap_or(frwd);
            if frwd_pos < frwd {
                fd = Some((i + 1) as i32);
                frwd = frwd_pos;
            }
            let bckwd_pos: usize = reversed_line.find(REV_DIGITS[i]).unwrap_or(bckwd);
            if bckwd_pos < bckwd {
                bd = Some((i + 1) as i32);
                bckwd = bckwd_pos;
            }
        }

        fd.unwrap() * 10 + bd.unwrap()
    }

    pub fn solve(input: &str) -> i32 {
        let mut total_calibration: i32 = 0;

        input
            .lines()
            .for_each(|line: &str| total_calibration += get_calibration(line));

        total_calibration
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::part1;
    use super::part2;

    const CASE1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const CASE2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test1() {
        assert_eq!(part1::solve(CASE1), 142.into());
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve(CASE2), 281.into());
    }
}
