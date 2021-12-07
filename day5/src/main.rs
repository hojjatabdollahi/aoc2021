use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    begin: Point,
    end: Point,
}

fn count_between(a: usize, b: usize) -> Vec<usize> {
    if a < b {
        (a..=b).into_iter().collect()
    } else {
        (b..=a).into_iter().rev().collect::<Vec<usize>>()
    }
}

fn get_point_between(p1: Point, p2: Point, allow_diagonal: bool) -> Vec<Point> {
    if p1.x == p2.x {
        count_between(p1.y, p2.y)
            .iter()
            .map(|y| Point { x: p1.x, y: *y })
            .collect()
    } else if p1.y == p2.y {
        count_between(p1.x, p2.x)
            .iter()
            .map(|x| Point { x: *x, y: p1.y })
            .collect()
    } else if allow_diagonal {
        count_between(p1.x, p2.x)
            .iter()
            .zip(count_between(p1.y, p2.y).iter())
            .map(|(x, y)| Point { x: *x, y: *y })
            .collect::<Vec<Point>>()
    } else {
        vec![]
    }
}

impl FromStr for Line {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(" -> ").unwrap();
        Ok(Self {
            begin: {
                let s = s.0.split_once(',').unwrap();
                Point {
                    x: s.0.parse().unwrap(),
                    y: s.1.parse().unwrap(),
                }
            },
            end: {
                let s = s.1.split_once(',').unwrap();
                Point {
                    x: s.0.parse().unwrap(),
                    y: s.1.parse().unwrap(),
                }
            },
        })
    }
}

fn process(input: &str, diagonal: bool) -> usize {
    const GRID_SIZE: usize = 1000;
    let mut grid = vec![0; GRID_SIZE * GRID_SIZE];
    let _ = input
        .lines()
        .map(|l| Line::from_str(l).unwrap())
        .map(|l| {
            get_point_between(l.begin, l.end, diagonal)
                .iter()
                .map(|p| {
                    grid[p.x + p.y * GRID_SIZE] += 1;
                })
                .collect::<()>()
        })
        .collect::<()>();

    grid.iter().fold(0, |acc, x| acc + (*x >= 2) as usize)
}

fn main() {
    println!(
        "part 1 : {}",
        process(
            &std::fs::read_to_string("input").expect("couldn't read the input"),
            false
        )
    );

    println!(
        "part 2 : {}",
        process(
            &std::fs::read_to_string("input").expect("couldn't read the input"),
            true
        )
    );
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
        assert_eq!(5, process(input, false));
    }

    #[test]
    fn part2() {
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
        assert_eq!(12, process(input, true));
    }

    #[test]
    fn get_point_between_test() {
        println!(
            "{:?}",
            get_point_between(Point { x: 1, y: 3 }, Point { x: 3, y: 3 }, true)
        );
    }
}
