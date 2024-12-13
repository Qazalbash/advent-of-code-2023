/// I was unable to solve this problem although the solution written here is from the explanation
/// shared in the video at https://youtu.be/glNiVe_Rztg?si=VGECb0KqmNGGJSmC.
mod input;
use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);
type CoordToCC = HashMap<Coord, i32>; // Coordinate to Connected Component
type CCToCoordSet = HashMap<i32, HashSet<Coord>>; // Connected Component to Coordinate Set

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs(i: i32, j: i32, k: i32, c: char, grid: &Vec<Vec<char>>, cc: &mut CoordToCC) {
    if i >= grid.len() as i32 || j >= grid.len() as i32 || i < 0 || j < 0 {
        return;
    }
    if grid[i as usize][j as usize] != c {
        return;
    }
    if (*cc).contains_key(&(i, j)) {
        return;
    }
    cc.insert((i, j), k);
    for (dx, dy) in DIRECTIONS.iter() {
        let x: i32 = i + dx;
        let y: i32 = j + dy;
        dfs(x, y, k, c, grid, cc);
    }
}

fn connected_components(grid: &Vec<Vec<char>>) -> CCToCoordSet {
    let mut cc: CoordToCC = HashMap::new();
    let (n, m): (usize, usize) = (grid.len(), grid[0].len());
    let mut k: i32 = 0;
    for i in 0..n {
        for j in 0..m {
            let i: i32 = i as i32;
            let j: i32 = j as i32;
            if !cc.contains_key(&(i, j)) {
                dfs(i, j, k, grid[i as usize][j as usize], &grid, &mut cc);
                k += 1;
            }
        }
    }

    let mut ccs = CCToCoordSet::new();
    for (coord, c) in cc.iter() {
        ccs.entry(*c).or_insert(HashSet::new()).insert(*coord);
    }
    ccs
}

pub mod part1 {

    use super::{connected_components, DIRECTIONS};

    pub fn solve(input: &str) -> u32 {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect())
            .collect();
        let ccs = connected_components(&grid);
        let (n, m): (usize, usize) = (grid.len(), grid[0].len());
        let mut total_cost: u32 = 0;

        for (_, coords) in ccs.iter() {
            let mut perimeter: u32 = 0;
            for coord in coords.iter() {
                let (i, j) = coord;
                for (dx, dy) in DIRECTIONS.iter() {
                    let x: i32 = i + dx;
                    let y: i32 = j + dy;
                    if (!coords.contains(&(x, y)))
                        || x < 0
                        || x >= n as i32
                        || y < 0
                        || y >= m as i32
                    {
                        perimeter += 1;
                    }
                }
            }
            let area: u32 = coords.len() as u32;
            total_cost += area * perimeter;
        }
        total_cost
    }
}

pub mod part2 {

    use super::{connected_components, Coord, HashSet, DIRECTIONS};

    pub fn solve(input: &str) -> u32 {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect())
            .collect();
        let ccs = connected_components(&grid);
        let (n, m): (usize, usize) = (grid.len(), grid[0].len());
        let mut total_cost: u32 = 0;

        for (_, coords) in ccs.iter() {
            let mut perimeter_set: HashSet<(Coord, Coord)> = HashSet::new();
            for coord in coords.iter() {
                let (i, j) = coord;
                for (dx, dy) in DIRECTIONS.iter() {
                    let x: i32 = i + dx;
                    let y: i32 = j + dy;
                    if (!coords.contains(&(x, y)))
                        || x < 0
                        || x >= n as i32
                        || y < 0
                        || y >= m as i32
                    {
                        perimeter_set.insert((coord.to_owned(), (x, y)));
                    }
                }
            }
            let mut perimeter_set2: HashSet<(Coord, Coord)> = HashSet::new();
            for (p1, p2) in perimeter_set.iter() {
                let mut keep: bool = true;
                for (dx, dy) in [(1, 0), (0, 1)].iter() {
                    let p1n: Coord = (p1.0 + dx, p1.1 + dy);
                    let p2n: Coord = (p2.0 + dx, p2.1 + dy);
                    if perimeter_set.contains(&(p1n.to_owned(), p2n.to_owned())) {
                        keep = false;
                        break;
                    }
                }
                if keep {
                    perimeter_set2.insert((p1.to_owned(), p2.to_owned()));
                }
            }
            let area: u32 = coords.len() as u32;
            total_cost += area * (perimeter_set2.len() as u32);
        }
        total_cost
    }
}

#[cfg(test)]
mod tests {
    use super::{input, part1, part2};
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC"};

    const EXAMPLE2: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE"};

    #[test]
    fn test1_example() {
        assert_eq!(part1::solve(EXAMPLE1), 140);
        assert_eq!(part1::solve(EXAMPLE2), 1930);
    }

    #[test]
    fn test2_example() {
        assert_eq!(part2::solve(EXAMPLE1), 80);
        assert_eq!(part2::solve(EXAMPLE2), 1206);
    }

    #[test]
    fn test1() {
        assert_eq!(part1::solve(input::CASE), 1457298);
    }

    #[test]
    fn test2() {
        assert_eq!(part2::solve(input::CASE), 921636);
    }
}
