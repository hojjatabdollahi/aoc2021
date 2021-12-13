use std::fs;

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(line: &str) -> Direction {
    let dir = line.split(' ').next().unwrap();
    let amount = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
    if dir.starts_with("forward") {
        Direction::Forward(amount)
    } else if line.starts_with("up") {
        Direction::Up(amount)
    } else {
        Direction::Down(amount)
    }
}

fn main() {
    
    // part 1
    let output = fs::read_to_string("input").expect("Couldn't read the input")
        .lines()
        .map(|line| {
            parse(line)
        })
        .fold((0, 0), |acc, dir| {
            match dir {
                Direction::Forward(x) => {(acc.0 + x, acc.1)},
                Direction::Up(y) => {(acc.0, acc.1 - y)},
                Direction::Down(y) => {(acc.0, acc.1 + y)},
            }
        });
    dbg!(output.0*output.1);

    // part 2
    let output = fs::read_to_string("input").expect("Couldn't read the input")
        .lines()
        .map(|line| {
            parse(line)
        })
        .fold((/* horizontal: */ 0, /* depth: */ 0, /* aim: */ 0 ), |acc, dir| {
            match dir {
                Direction::Forward(x) => {(acc.0 + x, acc.1 + acc.2*x, acc.2)},
                Direction::Up(x) => {(acc.0, acc.1, acc.2 - x)},
                Direction::Down(x) => {(acc.0, acc.1, acc.2 + x)},
            }
        });
    dbg!(output.0*output.1);

}
