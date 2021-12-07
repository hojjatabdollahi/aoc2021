fn process(input: &str, daynum: usize) -> usize {
    let mut pool = [0; 9];
    let _ = input
        .trim()
        .split(",")
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| pool[x] += 1)
        .collect::<()>();

    let _ = (0..daynum)
        .into_iter()
        .map(|_| {
            let new = pool[0];
            for i in 0..8 {
                pool[i] = pool[i + 1];
            }
            pool[6] += new;
            pool[8] = new;
        })
        .collect::<()>();

    pool.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    println!(
        "Part 1: {:?}",
        process(
            &std::fs::read_to_string("input").expect("Couldn't open input"),
            80,
        )
    );
    println!(
        "Part 2: {:?}",
        process(
            &std::fs::read_to_string("input").expect("Couldn't open input"),
            256,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "3,4,3,1,2";
        assert_eq!(5934, process(input, 80));
    }
}
