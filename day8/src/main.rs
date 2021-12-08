fn solve(observations: Vec<&str>, input: Vec<&str>) -> usize {
    let one = observations
        .iter()
        .filter(|o| o.len() == 2)
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let four = observations
        .iter()
        .filter(|o| o.len() == 4)
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let seven = observations
        .iter()
        .filter(|o| o.len() == 3)
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let eight = observations
        .iter()
        .filter(|o| o.len() == 7)
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    // a
    let a = seven.iter().filter(|ch| !one.contains(ch)).next().unwrap();


    let three = observations
        .iter()
        .filter(|o| o.len() == 5 && one.iter().all(|ch| o.chars().any(|oc| oc == *ch)))
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    // b
    let b = four.iter().filter(|ch| !three.contains(ch)).next().unwrap();

    let five = observations
        .iter()
        .filter(|o| o.len() == 5 && o.chars().any(|ch| ch == *b))
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    //f => in 5 and 7 but  (we know a)
    let f = seven
        .iter()
        .filter(|ch| five.contains(ch))
        .filter(|ch| !(a == *ch))
        .next()
        .unwrap();

    // c => in 1 (we know f)
    let c = one.iter().filter(|ch| !(f == *ch)).next().unwrap();

    // d => in 4 not 7 and we know b
    let d = four
        .iter()
        .filter(|ch| !seven.contains(ch))
        .filter(|ch| !(b == *ch))
        .next()
        .unwrap();

    // g => in 3 not 4 and we know a
    let g = three
        .iter()
        .filter(|ch| !four.contains(ch))
        .filter(|ch| !(a == *ch))
        .next()
        .unwrap();

    // e => in 8 and not 3 and we know b
    let e = eight
        .iter()
        .filter(|ch| !three.contains(ch))
        .filter(|ch| !(b == *ch))
        .next()
        .unwrap();

    let zero = observations
        .iter()
        .filter(|o| o.len() == 6)
        .filter(|o| {
            let mut res = true;
            for i in [a, b, c, e, f, g] {
                res &= o.chars().collect::<Vec<_>>().contains(i)
            }
            res
        })
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let two = observations
        .iter()
        .filter(|o| o.len() == 5)
        .filter(|o| {
            let mut res = true;
            for i in [a, c, d, e, g] {
                res &= o.chars().collect::<Vec<_>>().contains(i)
            }
            res
        })
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let six = observations
        .iter()
        .filter(|o| o.len() == 6)
        .filter(|o| {
            let mut res = true;
            for i in [a, b, d, e, f, g] {
                res &= o.chars().collect::<Vec<_>>().contains(i)
            }
            res
        })
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    let nine = observations
        .iter()
        .filter(|o| o.len() == 6)
        .filter(|o| {
            let mut res = true;
            for i in [a, b, c, d, f, g] {
                res &= o.chars().collect::<Vec<_>>().contains(i)
            }
            res
        })
        .collect::<Vec<_>>()[0]
        .chars()
        .collect::<Vec<_>>();

    input.iter().fold(0, |acc, digit| {
        let mut result = 0;
        for i in [(&zero, 0),(&one,1),(&two,2),(&three,3),(&four,4),(&five,5),(&six,6),(&seven,7),(&eight,8),(&nine,9)] {
            if digit.len() == i.0.len() {
                if digit.chars().all(|ch| i.0.contains(&ch)) {
                    result = i.1; 
                    break;
                }
            }
        }
        acc*10+result
    })
}

fn process1(input: &str) -> usize {
    let uniq = vec![2, 3, 4, 7];
    input
        .lines()
        .into_iter()
        .map(|l| {
            l.split('|').collect::<Vec<_>>()[1]
                .trim()
                .split(' ')
                .map(|s| if uniq.contains(&s.len()) { 1 } else { 0 })
                .collect::<Vec<usize>>()
                .iter()
                .fold(0, |acc, n| acc + n)
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(0, |acc, n| acc + n)
}

fn process2(input: &str) -> usize {
    input.lines().into_iter().fold(0, |acc, l| {
        let mut l = l.split('|');
        let observations = l.next().unwrap().split(' ').collect();
        let test = l.next().unwrap().split(' ').collect();
        acc+solve(observations, test)
    })


}

fn main() {
    println!(
        "part 1: {:?}",
        process1(&std::fs::read_to_string("input").expect("Couldn't read input"))
    );
    println!(
        "part 2: {:?}",
        process2(&std::fs::read_to_string("input").expect("Couldn't read input"))
    );
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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

    #[test]
    fn solve_test() {
        let obs = "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec"
            .split(' ')
            .collect::<Vec<&str>>();
        let input = "fcgedb cgb dgebacf gc".split(' ').collect::<Vec<&str>>();
        assert_eq!(9781, solve(obs, input))
    }

    #[test]
    fn part2() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        assert_eq!(61229, process2(input));
    }
}
