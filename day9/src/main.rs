fn process1(input: &str) -> u32 {
    let l = input
        .lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total_minima = 0;
    for i in 0..l.len() {
        for j in 0..l[0].len() {
            let mut minima = true;
            if i as i32 - 1 >= 0 && l[i - 1][j] <= l[i][j] {
                minima &= false;
            } else if i + 1 < l.len() && l[i + 1][j] <= l[i][j] {
                minima &= false;
            } else if j as i32 - 1 >= 0 && l[i][j - 1] <= l[i][j] {
                minima &= false;
            } else if j + 1 < l[0].len() && l[i][j + 1] <= l[i][j] {
                minima &= false;
            }
            if minima {
                total_minima += l[i][j] + 1;
            }
        }
    }
    total_minima
}
fn main() {
    println!(
        "Part 1: {}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read the inptu"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        assert_eq!(15, process1(input));
    }
}
