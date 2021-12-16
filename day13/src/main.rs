enum Axis {
    X,
    Y,
}
fn process1(input: &str, part1: bool) -> usize {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots_str
        .lines()
        .into_iter()
        .map(|c| c.split_once(',').unwrap())
        .map(|c| (c.0.parse().unwrap(), c.1.parse().unwrap()))
        .collect();
    let max_x = dots.iter().max_by_key(|c| c.0).unwrap().0 + 1;
    let max_y = dots.iter().max_by_key(|c| c.1).unwrap().1 + 1;
    let mut grid = vec![vec![false; max_x]; max_y];
    for dot in dots {
        grid[dot.1][dot.0] = true;
    }

    let mut current_max_x = max_x;
    let mut current_max_y = max_y;

    for fold in folds_str.lines() {
        let (axis, value) = fold.split_once('=').unwrap();
        let axis = if axis == "fold along y" {
            Axis::Y
        } else {
            Axis::X
        };
        let value = value.parse::<usize>().unwrap();
        match axis {
            Axis::X => {
                if value < current_max_x / 2 {
                    panic!("bottom half is bigger than the top");
                }
                for i in 0..current_max_y {
                    for j in value..current_max_x {
                        grid[i][2 * value - j] |= grid[i][j];
                    }
                }
                current_max_x = value;
            }
            Axis::Y => {
                if value < current_max_y / 2 {
                    panic!("right half is bigger than the left");
                }
                for i in value..current_max_y {
                    for j in 0..current_max_x {
                        grid[2 * value - i][j] |= grid[i][j];
                    }
                }
                current_max_y = value;
            }
        }
        if part1 {
            break;
        }
    }
    let mut counter = 0;
    for i in 0..current_max_y {
        for j in 0..current_max_x {
            if grid[i][j] {
                if !part1 {
                    print!("#");
                }
                counter += 1;
            } else {
                if !part1 {
                    print!(".");
                }
            }
        }
        if !part1 {
            println!("");
        }
    }

    counter
}

fn main() {
    println!(
        "Part 1: {}",
        process1(
            &std::fs::read_to_string("input").expect("Couldn't read the input"),
            true
        )
    );
    println!(
        "Part 2: {}",
        process1(
            &std::fs::read_to_string("input").expect("Couldn't read the input"),
            false
        )
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
        assert_eq!(17, process1(input, true));
    }
}
