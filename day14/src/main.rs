use std::collections::HashMap;

fn process1(input: &str, steps: usize) -> usize {
    let (input, dataset) = input.split_once("\n\n").unwrap();
    let dataset: HashMap<&str, &str> = dataset
        .lines()
        .into_iter()
        .map(|f| f.split_once(" -> ").unwrap())
        .collect();
    let mut input = input.chars().collect::<Vec<char>>();
    for _ in 0..steps {
        let res = input
            .windows(2)
            .map(|f| {
                vec![
                    f.to_owned()[0],
                    dataset
                        .get(f.iter().collect::<String>().as_str())
                        .unwrap()
                        .chars()
                        .collect::<Vec<char>>()[0],
                    f.to_owned()[1],
                ]
            })
            .collect::<Vec<_>>();
        input = vec![];
        for i in 0..res.len() {
            if i == res.len() - 1 {
                input.push(res[i][0]);
                input.push(res[i][1]);
                input.push(res[i][2]);
            } else {
                input.push(res[i][0]);
                input.push(res[i][1]);
            }
        }
    }
    let mut final_count = HashMap::new();
    for ch in input {
        *final_count.entry(ch).or_insert(0) += 1;
    }
    let max = final_count.iter().max_by_key(|f| f.1).unwrap().1;
    let min = final_count.iter().min_by_key(|f| f.1).unwrap().1;
    (max - min) as usize
}

fn process2(input: &str, steps: usize) -> usize {
    let (input, dataset_str) = input.split_once("\n\n").unwrap();
    let dataset: HashMap<&str, &str> = dataset_str
        .lines()
        .into_iter()
        .map(|f| f.split_once(" -> ").unwrap())
        .collect();

    let mut output: HashMap<&str, usize> = dataset_str
        .lines()
        .into_iter()
        .map(|f| (f.split_once(" -> ").unwrap().0, 0))
        .collect();

    let input = input.chars().collect::<Vec<char>>();
    let _res = input
        .windows(2)
        .map(|f| {
            let ch = dataset
                .get(f.iter().collect::<String>().as_str())
                .unwrap()
                .chars()
                .collect::<Vec<char>>()[0];
            *output
                .get_mut(
                    vec![f.to_owned()[0], ch]
                        .iter()
                        .collect::<String>()
                        .as_str(),
                )
                .unwrap() += 1;
            *output
                .get_mut(
                    vec![ch, f.to_owned()[1]]
                        .iter()
                        .collect::<String>()
                        .as_str(),
                )
                .unwrap() += 1;
        })
        .collect::<Vec<_>>();
    let mut final_count = HashMap::new();
    for _ in 1..steps {
        let mut new_output: HashMap<&str, usize> = dataset_str
            .lines()
            .into_iter()
            .map(|f| (f.split_once(" -> ").unwrap().0, 0))
            .collect();
        for combo in dataset.iter() {
            let ch = dataset.get(combo.0).unwrap().chars().collect::<Vec<char>>()[0];
            let repeat = *output.get(combo.0).unwrap();
            if repeat > 0 {
                *new_output
                    .get_mut(
                        vec![combo.0.chars().collect::<Vec<_>>()[0], ch]
                            .iter()
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap() += repeat;
                *new_output
                    .get_mut(
                        vec![ch, combo.0.chars().collect::<Vec<_>>()[1]]
                            .iter()
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap() += repeat;
            }
        }
        output = new_output;
    }
    for ch in output.iter() {
        *final_count
            .entry(ch.0.chars().collect::<Vec<_>>()[0])
            .or_insert(0) += ch.1;
        *final_count
            .entry(ch.0.chars().collect::<Vec<_>>()[1])
            .or_insert(0) += ch.1;
    }
    // add an extra 1 to the beginning character and the end character, since they don't repeat
    *final_count.entry(*input.first().unwrap()).or_insert(0) += 1;
    *final_count.entry(*input.last().unwrap()).or_insert(0) += 1;
    let max = final_count.iter().max_by_key(|f| f.1).unwrap().1;
    let min = final_count.iter().min_by_key(|f| f.1).unwrap().1;
    println!("Max: {}, Min: {}", max, min);
    // now we have 2 of every character, so we divide by two
    (max - min) as usize / 2
}
fn main() {
    println!(
        "Part 1: {:?}",
        process1(
            &std::fs::read_to_string("input").expect("couldn't read the input"),
            10
        )
    );
    println!(
        "Part 2: {:?}",
        process2(
            &std::fs::read_to_string("input").expect("couldn't read the input"),
            40
        )
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(1588, process2(input, 10));
    }
    #[test]
    fn part2() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(2188189693529, process2(input, 40));
    }
}
