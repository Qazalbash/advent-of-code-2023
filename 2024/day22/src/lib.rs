mod input;

fn mix(x: u64, y: u64) -> u64 {
    x ^ y
}

fn prune(x: u64) -> u64 {
    x % 16777216
}

fn next_step(x: u64) -> u64 {
    let r: u64 = prune(mix(x << 6, x));
    let s: u64 = prune(mix(r >> 5, r));
    let t: u64 = prune(mix(s << 11, s));
    t
}

pub mod part1 {

    use super::next_step;

    pub fn solve(secret_numbers: Vec<u64>) -> u64 {
        let mut acc: u64 = 0;
        const N: usize = 2000;
        secret_numbers.iter().for_each(|&x| {
            let mut y = x;
            for _ in 0..N {
                y = next_step(y);
            }
            acc += y;
        });
        acc
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::{input, next_step, part1};

    #[test]
    fn test_next_step() {
        let secret_number: u64 = 123;
        let evolution: Vec<u64> = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut x = secret_number;
        for i in 0..10 {
            x = next_step(x);
            assert_eq!(x, evolution[i]);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve(input::input()), 17163502021);
    }
}
