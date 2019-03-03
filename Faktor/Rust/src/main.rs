use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u32> = input.trim().split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

    if nums[0] == 1 {
        println!("{}", nums[1]);
    }
    else {
        println!("{}", nums[0] * (nums[1] - 1) + 1);
    }
}
