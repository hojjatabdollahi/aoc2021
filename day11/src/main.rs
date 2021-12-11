use std::{str::FromStr};
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
#[derive(Debug)]
struct Grid {
    grid : [[u32; 10];10],
}
impl FromStr for Grid {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
       Err(WrongNumberOfColumns) 
    }
}
fn main() {
    eprintln!("{:?}",Grid::from_str("Hello, world!"));
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
    }
}
