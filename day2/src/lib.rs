pub mod part1 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    pub fn solve(input: &str) -> i32 {
        let mut total: i32 = 0;
        for line in input.lines() {
            let mut game: std::str::Split<'_, &str> = line.split(":");

            let id: i32 = game
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            total += id;

            let parts: std::str::Split<'_, &str> = game.next().unwrap().trim().split(";");
            let mut impossible: bool = false;

            for part in parts {
                let colors: std::str::Split<'_, &str> = part.trim().split(",");
                for color in colors {
                    let mut color_explode: std::str::Split<'_, &str> = color.trim().split(" ");
                    let count: i32 = color_explode.next().unwrap().parse::<i32>().unwrap();
                    let color: &str = color_explode.next().unwrap();
                    if (color == "red" && count > MAX_RED)
                        || (color == "green" && count > MAX_GREEN)
                        || (color == "blue" && count > MAX_BLUE)
                    {
                        total -= id;
                        impossible = true;
                        break;
                    }
                }
                if impossible {
                    break;
                }
            }
        }
        total
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> i32 {
        let mut power: i32 = 0;
        for line in input.lines() {
            let mut game: std::str::Split<'_, &str> = line.split(":");

            game.next();

            let parts: std::str::Split<'_, &str> = game.next().unwrap().trim().split(";");

            let mut red: i32 = i32::MIN;
            let mut green: i32 = i32::MIN;
            let mut blue: i32 = i32::MIN;

            for part in parts {
                let colors: std::str::Split<'_, &str> = part.trim().split(",");
                for color in colors {
                    let mut color_explode: std::str::Split<'_, &str> = color.trim().split(" ");
                    let count: i32 = color_explode.next().unwrap().parse::<i32>().unwrap();
                    let color: &str = color_explode.next().unwrap();
                    if color == "red" {
                        red = std::cmp::max(red, count);
                    } else if color == "green" {
                        green = std::cmp::max(green, count);
                    } else {
                        blue = std::cmp::max(blue, count);
                    }
                }
            }
            power += red * green * blue;
        }
        power
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;
    use indoc::indoc;

    const CASE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test1() {
        assert_eq!(part1::solve(CASE), 8.into());
    }
    #[test]
    fn test2() {
        assert_eq!(part2::solve(CASE), 2286.into());
    }
}
