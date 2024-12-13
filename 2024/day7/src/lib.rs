mod input;

pub mod part1 {

    fn evaluate(target: u128, row: Vec<u128>) -> bool {
        let n: usize = row.len();
        if n == 1 {
            return target == row[0];
        }
        if target < row[n - 1] {
            return false;
        }
        let mut eq_true: bool = false;
        let last_element: u128 = row[n - 1];
        if target % last_element == 0 {
            eq_true = eq_true || evaluate(target / last_element, row[..n - 1].to_vec());
        }
        if eq_true {
            return true;
        }
        eq_true || evaluate(target - last_element, row[..n - 1].to_vec())
    }

    pub fn solve(input: Vec<Vec<u128>>) -> u128 {
        input
            .iter()
            .map(
                |row: &Vec<u128>| match evaluate(row[0], row[1..].to_vec()) {
                    true => row[0],
                    false => 0,
                },
            )
            .sum()
    }
}

pub mod part2 {
    fn evaluate(target: u128, row: Vec<u128>) -> bool {
        let n: usize = row.len();
        if n == 1 {
            return target == row[0];
        }
        if target < row[n - 1] {
            return false;
        }
        let mut eq_true: bool = false;
        let last_element: u128 = row[n - 1];
        if target % last_element == 0 {
            eq_true = eq_true || evaluate(target / last_element, row[..n - 1].to_vec());
        }
        if eq_true {
            return true;
        }
        eq_true = eq_true || evaluate(target - last_element, row[..n - 1].to_vec());
        if eq_true {
            return true;
        }
        let last_element_length: u128 = last_element.to_string().len() as u128;
        let target_length: u128 = target.to_string().len() as u128;
        if target_length > last_element_length {
            if target % 10u128.pow(last_element_length as u32) == last_element {
                eq_true = eq_true
                    || evaluate(
                        target / 10u128.pow(last_element_length as u32),
                        row[..n - 1].to_vec(),
                    );
            }
        }
        eq_true
    }

    pub fn solve(input: Vec<Vec<u128>>) -> u128 {
        input
            .iter()
            .map(
                |row: &Vec<u128>| match evaluate(row[0], row[1..].to_vec()) {
                    true => row[0],
                    false => 0,
                },
            )
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::input;
    use super::part1;
    use super::part2;

    #[test]
    fn test1() {
        assert_eq!(part1::solve(input::input()), 2437272016585);
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve(input::input()), 162987117690649);
    }
}
