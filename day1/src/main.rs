// Advent of Code 1
//
use std::fs;
fn main() {
    // day 1, question 1
    let content = fs::read_to_string("input")
        .expect("Couldn't read the input")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>();

    println!("P1: Using windows and folding {:?}",content
             .windows(2)
             .fold(0,|acc, w| acc + (w[1]>w[0]) as i32 ));

    println!("P1: Uisng just folding {:?}",content
             .iter()
             .fold((0,None), |acc, x| 
                  match acc {
                      (counter, Some(value)) if value < x => (counter+1, Some(x)), // increase
                      (_, None) => (0, Some(x)), // happens for first item
                      (counter, _) => (counter, Some(x)), // don't increase
                  }
                ).0);


    // day1, question 2
    println!("P2: Using windows and folding {:?}",content
             .windows(4)
             .fold(0,|acc, w| acc + (w[3]>w[0]) as i32 )); // you only need to compare the 4th and the first

    println!("P2: Using just folding {:?}",content
             .iter()
             .fold((0,None, None, None), |acc, x| 
                  match acc {
                      (counter, Some(a), Some(b), Some(c)) if a < x => (counter+1, Some(b), Some(c), Some(x)), // increase
                      (_, None, b, c) => (0, b, c, Some(x)), // happens for first item
                      (counter, _, b, c) => (counter, b, c, Some(x)), // don't increase
                  }
                ).0);
    
} 
