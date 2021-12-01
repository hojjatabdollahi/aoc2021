// Advent of Code 1
//
use std::fs;
fn main() {
    // day 1, question 1
    let content = fs::read_to_string("input").expect("Couldn't read the input");
    println!("{:?}",content
             .split('\n')
             .into_iter()
             .filter(|str| !str.parse::<i32>().is_err())
             .map(|str| str.parse::<i32>().unwrap())
             .fold((0,None), |acc, x| 
                  match acc {
                      (counter, Some(value)) if value < x => (counter+1, Some(x)), // increase
                      (_, None) => (0, Some(x)), // happens for first item
                      (counter, _) => (counter, Some(x)), // don't increase
                  }
                ).0);

    // day1, question 2
    println!("{:?}",content
             .split('\n')
             .into_iter()
             .filter(|str| !str.parse::<i32>().is_err())
             .map(|str| str.parse::<i32>().unwrap())
             .fold((0,None, None, None), |acc, x| 
                  match acc {
                      (counter, Some(a), Some(b), Some(c)) if a < x => (counter+1, Some(b), Some(c), Some(x)), // increase
                      (_, None, b, c) => (0, b, c, Some(x)), // happens for first item
                      (counter, a, b, c) => (counter, b, c, Some(x)), // don't increase
                  }
                ).0);
    
} 
