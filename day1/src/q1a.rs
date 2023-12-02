fn get_calibration(line: &str) -> i32 {
    let mut calibration: i32 = 0;
    let n: usize = line.len();
    let c: Vec<char> = line.chars().collect();
    let mut frwd: usize = 0;
    while frwd < n {
        if c[frwd].is_digit(10) {
            let num: i32 = c[frwd].to_digit(10).unwrap() as i32;
            calibration += num * 10;
            break;
        }
        frwd += 1;
    }

    let mut bckwd: usize = n - 1;
    while bckwd >= frwd {
        if c[bckwd].is_digit(10) {
            let num: i32 = c[bckwd].to_digit(10).unwrap() as i32;
            calibration += num;
            break;
        }
        bckwd -= 1;
    }
    calibration
}

pub fn solve(input: &str) -> i32 {
    let mut total_calibration: i32 = 0;
    for line in input.lines() {
        total_calibration += get_calibration(line);
    }
    total_calibration
}
