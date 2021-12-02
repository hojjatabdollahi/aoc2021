use std::fs;

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse(line: &str) -> Direction {
    let dir = line.split(' ').nth(0).unwrap();
    let amount = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
    if dir.starts_with("forward") {
        return Direction::Forward(amount);
    } else if line.starts_with("up") {
        return Direction::Up(amount);
    } else {
        return Direction::Down(amount);
    }
}

fn main() {
    
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

}
