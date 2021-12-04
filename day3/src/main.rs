use std::fs;

fn process1(input: &Vec<&str>) -> usize {
    let item_length = input[0].len();
    let mut popularity_of_one = vec![0; item_length];
    for line in input {
        for i in 0..item_length {
            if line.chars().nth(i) == Some('1') {
                popularity_of_one[i] += 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    let max = input.len();

    for i in 0..item_length {
        if popularity_of_one[i] > max / 2 {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    to_usize(&gamma) * to_usize(&epsilon)
}

enum Device {
    Oxygen,
    CO2,
}

fn to_usize(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
}

fn reduce(input: &Vec<&str>, device: Device, current_index: usize) -> usize {
    if input.len() == 1 {
        return to_usize(input[0]);
    }
    let mut popularity_of_one = 0;
    for line in input {
        if line.chars().nth(current_index) == Some('1') {
            popularity_of_one += 1;
        }
    }
    let popular: char;
    match device {
        Device::Oxygen => {
            if popularity_of_one >= (input.len() + 1) / 2 {
                popular = '1';
            } else {
                popular = '0';
            }
        }
        Device::CO2 => {
            if popularity_of_one >= (input.len() + 1) / 2 {
                popular = '0';
            } else {
                popular = '1';
            }
        }
    }

    let mut output = vec![];

    for line in input {
        if line.chars().nth(current_index) == Some(popular) {
            output.push(*line);
        }
    }
    reduce(&output, device, current_index + 1)
}

fn process2(input: &Vec<&str>) -> usize {
    reduce(input, Device::Oxygen, 0) * reduce(input, Device::CO2, 0)
}

fn main() {
    println!(
        "{}",
        process1(
            &fs::read_to_string("input")
                .expect("couldn't read file")
                .split('\n')
                .collect::<Vec<&str>>()
        )
    );
    println!(
        "{}",
        process2(
            &fs::read_to_string("input")
                .expect("couldn't read file")
                .split('\n')
                .collect::<Vec<&str>>()
        )
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let test_input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(
            198,
            process1(&test_input.split('\n').collect::<Vec<&str>>())
        );
    }
    #[test]
    fn part2() {
        let test_input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
        assert_eq!(
            230,
            process2(&test_input.split('\n').collect::<Vec<&str>>())
        );
    }
}
