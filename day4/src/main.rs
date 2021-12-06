use std::{fs, num::ParseIntError, str::FromStr};

type Cols = Vec<usize>;
type Rows = Vec<usize>;

#[derive(Debug)]
struct Board {
    board: Vec<Vec<(usize, bool)>>,
    rows: Rows,
    cols: Cols,
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            board: s
                .lines()
                .into_iter()
                .map(|l| {
                    l.trim()
                        .split_ascii_whitespace()
                        .map(|n| (n.parse().unwrap(), false))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
            rows: vec![0; 5],
            cols: vec![0; 5],
        })
    }
}

impl Board {
    fn check(&mut self, num: usize) -> bool {
        'outer: for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].0 == num {
                    self.board[i][j].1 = true;
                    self.rows[i] += 1;
                    self.cols[j] += 1;
                    if self.rows[i] == 5 || self.cols[j] == 5 {
                        return true;
                    }
                    break 'outer;
                }
            }
        }
        false
    }
}

fn process1(input: &str) -> usize {
    let input = input.lines().collect::<Vec<&str>>();
    let random_numbers: Vec<usize> = input
        .iter()
        .next()
        .map(|f| f)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    for board_number in 0..((input.len() - 1) / 6) {
        boards.push(
            Board::from_str(&input[(2 + board_number * 6)..(7 + board_number * 6)].join("\n"))
                .unwrap(),
        );
    }

    let mut bingo_board = 0;
    let mut winning_random_number = 0;
    'outer: for next_random_number in &random_numbers {
        for i in 0..boards.len() {
            if boards[i].check(*next_random_number) {
                winning_random_number = *next_random_number;
                bingo_board = i;
                break 'outer;
            }
        }
    }

    boards[bingo_board]
        .board
        .iter()
        .map(|v| {
            v.iter()
                .filter(|x| !x.1)
                .map(|x| x.0)
                .fold(0, |acc, x| acc + x)
        })
        .fold(0, |acc, x| acc + x)
        * winning_random_number
}

fn process2(input: &str) -> usize {
    let input = input.lines().collect::<Vec<&str>>();
    let random_numbers: Vec<usize> = input
        .iter()
        .next()
        .map(|f| f)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    for board_number in 0..((input.len() - 1) / 6) {
        boards.push(
            Board::from_str(&input[(2 + board_number * 6)..(7 + board_number * 6)].join("\n"))
                .unwrap(),
        );
    }

    let mut bingo_board = 0;
    let mut winning_random_number = 0;
    let mut bingoed_boards = vec![false; boards.len()];
    let mut tot_boards = boards.len();

    'outer: for next_random_number in &random_numbers {
        for i in 0..boards.len() {
            if boards[i].check(*next_random_number) {
                winning_random_number = *next_random_number;
                bingo_board = i;
                if !bingoed_boards[i] {
                    bingoed_boards[i] = true;
                    tot_boards -= 1;
                }
                if tot_boards == 0 {
                    break 'outer;
                }
            }
        }
    }

    boards[bingo_board]
        .board
        .iter()
        .map(|v| {
            v.iter()
                .filter(|x| !x.1)
                .map(|x| x.0)
                .fold(0, |acc, x| acc + x)
        })
        .fold(0, |acc, x| acc + x)
        * winning_random_number
}
fn main() {
    println!(
        "part1: {}",
        process1(&fs::read_to_string("input").expect("couldn't read to string"))
    );
    println!(
        "part2: {}",
        process2(&fs::read_to_string("input").expect("couldn't read to string"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
        "#;
        assert_eq!(4512, process1(input));
    }

    #[test]
    fn part2() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
            8  2 23  4 24
            21  9 14 16  7
            6 10  3 18  5
            1 12 20 15 19

            3 15  0  2 22
            9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7
        "#;
        assert_eq!(1924, process2(input));
    }
}
