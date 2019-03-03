use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let sum: i32 = line.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .fold(0_i32, |acc: i32, num: i32| acc + num);
        println!("{}", sum / 2);
    }
}
