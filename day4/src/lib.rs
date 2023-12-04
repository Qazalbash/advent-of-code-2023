fn get_lists(line: &str) -> (Vec<i32>, Vec<i32>) {
    let nums_list: Vec<&str> = line.trim().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split('|')
        .collect::<Vec<&str>>();
    let winning_list: Vec<i32> = nums_list[0]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let holding_list: Vec<i32> = nums_list[1]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (winning_list, holding_list)
}

pub mod part1 {
    use super::get_lists;
    pub fn solve(input: &str) -> i32 {
        let mut total: i32 = 0;
        for line in input.lines() {
            let (winning_list, holding_list): (Vec<i32>, Vec<i32>) = get_lists(line);
            let mut count: usize = 0;
            for num in winning_list {
                if holding_list.contains(&num) {
                    count += 1;
                }
            }
            if count > 0 {
                total += 1 << (count - 1);
            }
        }
        total
    }
}

pub mod part2 {
    use super::get_lists;
    pub fn solve(input: &str) -> i32 {
        let n: usize = input.lines().count();
        let mut instances: Vec<usize> = vec![1; n];
        for (i, line) in input.lines().enumerate() {
            let (winning_list, holding_list) = get_lists(line);
            let mut count: usize = 0;
            for num in winning_list {
                if holding_list.contains(&num) {
                    count += 1;
                }
            }
            if count > 0 {
                let extra: usize = if count < n - i { 1 } else { (n - i) / count };

                for j in (i + 1)..(std::cmp::min(i + count + 1, n)) {
                    instances[j] += instances[i] * extra;
                }
            }
        }
        instances.iter().sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    use indoc::indoc;

    const CASE1: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test1a() {
        assert_eq!(part1::solve(CASE1), 13.into());
    }

    #[test]
    fn test2a() {
        assert_eq!(part2::solve(CASE1), 30.into());
    }
}
