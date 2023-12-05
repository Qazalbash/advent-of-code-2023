pub const DIRECTIONS: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub mod part1 {
    use super::DIRECTIONS;

    pub fn solve(input: &str) -> i32 {
        let mut total: i32 = 0;
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let total_rows: i32 = map.len() as i32;
        let total_cols: i32 = map[0].len() as i32;
        for r in 0..total_rows {
            let mut n: i32 = 0;
            let mut has_part: bool = false;
            for c in 0..total_cols {
                if map[r as usize][c as usize].is_ascii_digit() {
                    n = n * 10 + map[r as usize][c as usize].to_digit(10).unwrap() as i32;
                    for (rr, cc) in DIRECTIONS {
                        if 0 <= r + rr && r + rr < total_rows && 0 <= c + cc && c + cc < total_cols
                        {
                            let ch: char = map[(r + rr) as usize][(c + cc) as usize];
                            if !ch.is_ascii_digit() && ch != '.' {
                                has_part = true;
                            }
                        }
                    }
                } else if n > 0 {
                    if has_part {
                        total += n;
                    }
                    n = 0;
                    has_part = false;
                }
            }
        }
        total
    }
}

pub mod part2 {
    use super::DIRECTIONS;
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn solve(input: &str) -> i32 {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let total_rows: i32 = map.len() as i32;
        let total_cols: i32 = map[0].len() as i32;
        let mut nums: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        for r in 0..total_rows {
            let mut n: i32 = 0;
            let mut gears: HashSet<(i32, i32)> = HashSet::new();
            for c in 0..total_cols {
                if map[r as usize][c as usize].is_ascii_digit() {
                    n = n * 10 + map[r as usize][c as usize].to_digit(10).unwrap() as i32;
                    for (rr, cc) in DIRECTIONS {
                        if 0 <= r + rr
                            && r + rr < total_rows
                            && 0 <= c + cc
                            && c + cc < total_cols
                            && map[(r + rr) as usize][(c + cc) as usize] == '*'
                        {
                            gears.insert((r + rr, c + cc));
                        }
                    }
                } else if n > 0 {
                    gears
                        .iter()
                        .for_each(|(rr, cc)| nums.entry((*rr, *cc)).or_default().push(n));
                    n = 0;
                    gears.clear();
                }
            }
        }
        let mut total: i32 = 0;
        nums.iter().for_each(|(_, v)| {
            if v.len() == 2 {
                total += v[0] * v[1];
            }
        });
        total
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;
    use indoc::indoc;

    const CASE_A: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test1a() {
        assert_eq!(part1::solve(CASE_A), 4361.into());
    }

    #[test]
    fn test2a() {
        assert_eq!(part2::solve(CASE_A), 467835.into());
    }
}
