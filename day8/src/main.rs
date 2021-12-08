fn process1 (input: &str) -> usize {
    let uniq = vec![2,3,4,7];
    input
    .lines()
    .into_iter()
    .map(|l| 
        l.split('|')
        .collect::<Vec<_>>()[1]
        .trim()
        .split(' ')
        .map(|s| 
            if uniq.contains(&s.len()) {1} else {0})
            .collect::<Vec<usize>>()
            .iter()
            .fold(0, |acc, n| acc+n))
        .collect::<Vec<_>>()
        .iter()
        .fold(0, |acc, n| acc+n)

}
fn main() {
    println!("{:?}", 
        process1(&std::fs::read_to_string("input").expect("Couldn't read input")));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input= r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        assert_eq!(26, process1(input));
    }
}