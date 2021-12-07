use std::str::FromStr;
use std::io::ErrorKind::InvalidData;
#[derive(Debug)]
struct Line {
    begin: (usize, usize),
    end: (usize, usize),
}
impl FromStr for Line {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(" -> ").unwrap();
        Ok (
            Self {
                begin: {
                    let s = s.0.split_once(',').unwrap();
                    (s.0.parse().unwrap(), s.1.parse().unwrap())
                },
                end: {
                    let s = s.1.split_once(',').unwrap();
                    (s.0.parse().unwrap(), s.1.parse().unwrap())
                },
            }
        )
    }
}
fn process1(input: &str) -> usize {

    const GRID_SIZE : usize = 1000;
    let mut grid = vec![0;GRID_SIZE*GRID_SIZE];
    let _ = input.lines().map(|l| {
        Line::from_str(l).unwrap()
    }).map(|l|{
        if l.begin.0 == l.end.0 || l.begin.1 == l.end.1 {
            println!("{:?}", l);
            let (mut x1, mut y1) = l.begin;
            let (mut x2, mut y2) = l.end;

            if x1 > x2 {
                let tmp = x1;
                x1 = x2; 
                x2 = tmp;
            }
            if y1 > y2 {
                let tmp = y1;
                y1 = y2; 
                y2 = tmp;
            }

            for i in x1..=x2 {
                for j in y1..=y2 {
                    println!("{} {}", i, j);
                    grid[i+j*GRID_SIZE] += 1;
                }
            }
        }
    }).collect::<()>();

    println!("{:?}", grid);

    grid.iter().fold(0, |acc,x| acc + (*x>=2) as usize)
}

fn main() {
    println!("part 1 : {}", process1(&std::fs::read_to_string("input").expect("couldn't read the input")));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(5, process1(input));
    }
}
