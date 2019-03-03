use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_nums: Vec<u32> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();

    let max_length = ((input_nums[1].pow(2) + input_nums[2].pow(2)) as f64).sqrt();

    for i in 0..input_nums[0] {
        if (match_length() as f64) <= max_length {
            println!("DA");
        }
        else {
            println!("NE");
        }
    }
}

fn match_length() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
