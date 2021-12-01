// Advent of Code 1
//
use std::fs;
fn main() {
    //let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //for i in input.iter() {
    //    println!("{:?}",i);
    //}
    //let mut it = input.iter();
    //for i in 0..11 {
    //    println!("{:?}", it.next());
    //}
    let content = fs::read_to_string("input").expect("Couldn't read the input");
    println!("{:?}",content
             .split('\n')
             .into_iter()
             .filter(|str| !str.parse::<i32>().is_err())
             .map(|str| str.parse::<i32>().unwrap())
             .fold((0,None), |counter, x| 
                  match counter {
                      (counter, Some(value)) if value < x => (counter+1, Some(x)), // increase
                      (_, None) => (0, Some(x)), // happens for first item
                      (counter, _) => (counter, Some(x)), // don't increase
                  }
                ).0);
    
} 
