use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParsingError {
    #[error("Wrong number of rows...")]
    WrongNumberOfRows,
    #[error("Wrong number of cols...")]
    WrongNumberOfColumns,
}
use ParsingError::*;

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] io::Error),
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader {
//         expected: String,
//         found: String,
//     },
//     #[error("unknown data store error")]
//     Unknown,
// }
#[derive(Debug, Default)]
struct Grid {
    grid: [[(u32, bool); 10]; 10],
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|f| f
                    .iter()
                    .map(|f| f.0.to_string())
                    .collect::<Vec<String>>()
                    .concat())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Grid {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        if lines.len() != 10 {
            return Err(WrongNumberOfRows);
        }

        let mut res = Self::default();

        for i in 0..10 {
            let octopuses = lines[i]
                .chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            if octopuses.len() != 10 {
                return Err(WrongNumberOfColumns);
            }
            for j in 0..10 {
                res.grid[i][j] = (octopuses[j], false);
            }
        }
        Ok(res)
    }
}

fn increment(input: &mut Grid) {
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            input.grid[i][j].0 += 1;
        }
    }
}
fn reset(input: &mut Grid) {
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            input.grid[i][j].1 = false;
        }
    }
}

fn check(input: &mut Grid) -> usize {
    let mut res = 0;
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            if input.grid[i][j].0 > 9 {
                res += 1;
                input.grid[i][j] = (0, true);
                increment_neighbor(input, (i, j));
            }
        }
    }
    res
}

fn increment_neighbor(input: &mut Grid, coord: (usize, usize)) {
    // (x-1, y-1)
    if coord.0 as i32 - 1 >= 0 && coord.1 as i32 - 1 >= 0 && !input.grid[coord.0 - 1][coord.1 - 1].1
    {
        input.grid[coord.0 - 1][coord.1 - 1].0 += 1;
    }
    // (x-1, y)
    if coord.0 as i32 - 1 >= 0 && !input.grid[coord.0 - 1][coord.1].1 {
        input.grid[coord.0 - 1][coord.1].0 += 1;
    }
    // (x-1, y+1)
    if coord.0 as i32 - 1 >= 0
        && coord.1 as i32 + 1 < input.grid[0].len() as i32
        && !input.grid[coord.0 - 1][coord.1 + 1].1
    {
        input.grid[coord.0 - 1][coord.1 + 1].0 += 1;
    }

    // (x, y-1)
    if coord.1 as i32 - 1 >= 0 && !input.grid[coord.0][coord.1 - 1].1 {
        input.grid[coord.0][coord.1 - 1].0 += 1;
    }
    // (x, y+1)
    if coord.1 as i32 + 1 < input.grid[0].len() as i32 && !input.grid[coord.0][coord.1 + 1].1 {
        input.grid[coord.0][coord.1 + 1].0 += 1;
    }

    // (x+1, y-1)
    if coord.0 as i32 + 1 < input.grid.len() as i32
        && coord.1 as i32 - 1 >= 0
        && !input.grid[coord.0 + 1][coord.1 - 1].1
    {
        input.grid[coord.0 + 1][coord.1 - 1].0 += 1;
    }
    // (x+1,y)
    if coord.0 as i32 + 1 < input.grid.len() as i32 && !input.grid[coord.0 + 1][coord.1].1 {
        input.grid[coord.0 + 1][coord.1].0 += 1;
    }
    // (x+1, y+1)
    if coord.0 as i32 + 1 < input.grid.len() as i32
        && coord.1 as i32 + 1 < input.grid[0].len() as i32
        && !input.grid[coord.0 + 1][coord.1 + 1].1
    {
        input.grid[coord.0 + 1][coord.1 + 1].0 += 1;
    }
}

fn process1(input: &str, steps: u32) -> usize {
    let mut res = 0;
    let mut data = Grid::from_str(input).unwrap();
    for i in 0..steps {
        increment(&mut data);
        loop {
            let changed = check(&mut data);
            if changed == 0 {
                break;
            }
            res += changed;
        }
        reset(&mut data);
    }

    res
}
fn check_all_flash(input: &Grid) -> bool {
    for i in 0..input.grid.len() {
        for j in 0..input.grid[0].len() {
            if input.grid[i][j].0 != 0 {
                return false;
            }
        }
    }
    true
}
fn process2(input: &str) -> usize {
    let mut data = Grid::from_str(input).unwrap();
    for i in 0..usize::MAX {
        increment(&mut data);
        loop {
            if check(&mut data) == 0 {
                break;
            }
        }
        reset(&mut data);
        if check_all_flash(&data) {
            return i + 1;
        }
    }
    0
}
fn main() {
    println!(
        "Part 1: {}",
        process1(
            &std::fs::read_to_string("input").expect("Couldn't read the input"),
            100
        )
    );
    println!(
        "Part 2: {}",
        process2(&std::fs::read_to_string("input").expect("Couldn't read the input"))
    );
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn part1() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(1656, process1(input, 100));
    }
    #[test]
    fn part2() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(195, process2(input));
    }
    #[test]
    fn incerement_test() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        let mut a = Grid::from_str(input).unwrap();
        increment(&mut a);
        assert_eq!(6, a.grid[0][0].0);
    }

    #[test]
    fn incerement_neighbor_test() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        let mut a = Grid::from_str(input).unwrap();
        increment_neighbor(&mut a, (1, 1));
        println!("{}", a);
        assert_eq!(6, a.grid[0][0].0);
    }
}
