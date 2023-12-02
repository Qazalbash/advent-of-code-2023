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
        if c[frwd].is_digit(10) {
            fd = Some(c[frwd].to_digit(10).unwrap() as i32);
            break;
        }
        frwd += 1;
    }

    while frwd <= bckwd {
        if c[bckwd].is_digit(10) {
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

    for s in input.lines() {
        total_calibration += get_calibration(s);
    }

    total_calibration
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::solve;

    const CASE_B: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_b() {
        assert_eq!(solve(CASE_B), 281.into());
    }
}
