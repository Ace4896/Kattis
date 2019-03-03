use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u32> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    for i in 1..nums[2] + 1 {
        let div_by_x = i % nums[0] == 0;
        let div_by_y = i % nums[1] == 0;

        if div_by_x && div_by_y {
            println!("FizzBuzz");
        }
        else if div_by_x {
            println!("Fizz");
        }
        else if div_by_y {
            println!("Buzz");
        }
        else {
            println!("{}", i);
        }
    }
}
