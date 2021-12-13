use core::num;

fn process(input: &str) -> usize {
    let mut nums = input.trim().split(',').into_iter().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mid = nums.len()/2;
    nums.sort();
    let median = nums[mid] as i32;
    println!("Median: {}", median);
    nums.into_iter().fold(0, |acc, n| acc + (n as i32 - median).abs() as usize)

}

fn process2(input: &str) -> usize {
    let mut nums = input.trim().split(',').into_iter().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let avg = (nums.iter().fold(0, |acc, n| acc+n)/nums.len() )as i32;
    println!("Average: {}", avg);
    nums.into_iter().map(|n| (n as i32 - avg).abs()).fold(0, |acc, n| acc + ((n*(n+1)/2 ) as usize))

}
fn main() {
    println!("part1: {}", process(&std::fs::read_to_string("input").expect("couldn't read the input")));
    println!("part2: {}", process2(&std::fs::read_to_string("input").expect("couldn't read the input")));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, process(input));
    }
    fn part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(168, process2(input));
    }
}