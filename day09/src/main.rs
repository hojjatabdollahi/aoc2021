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

fn process2(input: &str) -> i32 {
    let l = input
        .lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut all_basins = vec![];
    for i in 0..l.len() {
        for j in 0..l[0].len() {
            let mut l2 = vec![vec![(false, false); l[0].len()]; l.len()];
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
                let mut to_seach = vec![];
                if i as i32 - 1 >= 0 {
                    to_seach.push((i - 1, j));
                }
                if i + 1 < l.len() {
                    to_seach.push((i + 1, j));
                }
                if j as i32 - 1 >= 0 {
                    to_seach.push((i, j - 1));
                }
                if j + 1 < l[0].len() {
                    to_seach.push((i, j + 1));
                }
                while to_seach.len() > 0 {
                    let current_cell = to_seach.pop().unwrap();
                    if !l2[current_cell.0][current_cell.1].0 {
                        // hasn't been checked yet
                        l2[current_cell.0][current_cell.1].0 = true;
                        if l[current_cell.0][current_cell.1] != 9 {
                            l2[current_cell.0][current_cell.1].1 = true; // part of the basin
                            if current_cell.0 as i32 - 1 >= 0
                                && !l2[current_cell.0 - 1][current_cell.1].0
                            {
                                to_seach.push((current_cell.0 - 1, current_cell.1));
                            }
                            if current_cell.0 + 1 < l.len()
                                && !l2[current_cell.0 + 1][current_cell.1].0
                            {
                                to_seach.push((current_cell.0 + 1, current_cell.1));
                            }
                            if current_cell.1 as i32 - 1 >= 0
                                && !l2[current_cell.0][current_cell.1 - 1].0
                            {
                                to_seach.push((current_cell.0, current_cell.1 - 1));
                            }
                            if current_cell.1 + 1 < l[0].len()
                                && !l2[current_cell.0][current_cell.1 + 1].0
                            {
                                to_seach.push((current_cell.0, current_cell.1 + 1));
                            }
                        }
                    }
                }
                all_basins.push(l2.iter().fold(0, |acc, row| {
                    acc + row.iter().fold(0, |acc2, cell| acc2 + (cell.1) as i32)
                }))
            }
        }
    }
    all_basins.sort();
    all_basins
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .windows(3)
        .take(1)
        .map(|f| f[0] * f[1] * f[2])
        .collect::<Vec<_>>()[0]
}
fn main() {
    println!(
        "Part 1: {}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read the inptu"))
    );
    println!(
        "Part 2: {}",
        process2(&std::fs::read_to_string("input").expect("Couldn't read the inptu"))
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
    #[test]
    fn part2() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
        assert_eq!(1134, process2(input));
    }
}
