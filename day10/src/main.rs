use std::collections::HashMap;

fn check_line1(line: &str) -> Option<usize> {
    let mut stack = Vec::new();
    let scores: HashMap<char, Option<usize>> = HashMap::from([
        (')', Some(3)),
        (']', Some(57)),
        ('}', Some(1197)),
        ('>', Some(25137)),
    ]);

    let closing: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for c in line.chars() {
        if "([{<".contains(c) {
            stack.push(c);
        } else {
            if closing.get(&stack.pop().unwrap()).unwrap() != &c {
                return *scores.get(&c).unwrap();
            }
        }
    }
    Some(0)
}

fn process1(input: &str) -> usize {
    input
        .lines()
        .map(|l| check_line1(l))
        .fold(0, |acc, x| acc + x.unwrap())
}

fn check_line2(line: &str) -> Option<usize> {
    let mut stack = Vec::new();
    let scores: HashMap<char, usize> = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let closing: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);

    for c in line.chars() {
        if "([{<".contains(c) {
            stack.push(c);
        } else {
            if closing.get(&stack.pop().unwrap()).unwrap() != &c {
                return None;
            }
        }
    }
    Some(
        (0..stack.len())
            .into_iter()
            .map(|_| stack.pop().unwrap())
            .fold(0, |acc, x| acc * 5 + scores.get(&x).unwrap()),
    )
}

fn process2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .map(|l| check_line2(l))
        .filter_map(|v| v)
        .collect::<Vec<_>>();
    let mid = scores.len() / 2;
    *scores.select_nth_unstable(mid).1
}

fn main() {
    println!(
        "Part 1: {}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read the input"))
    );
    println!(
        "Part 2: {}",
        process2(&std::fs::read_to_string("input").expect("Couldn't read the input"))
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(26397, process1(input));
    }
    #[test]
    fn part2() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(288957, process2(input));
    }
}
