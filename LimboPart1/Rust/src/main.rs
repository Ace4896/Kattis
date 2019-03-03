use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases: u8 = input.trim().parse().unwrap();
    
    for _ in 0..test_cases {
      input = String::new();
      io::stdin().read_line(&mut input).unwrap();
      let input_nums: Vec<u64> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();
      let n = input_nums[0] + input_nums[1];
      println!("{}", (((n * n) + n + 2) / 2) + input_nums[1]);
    }
}
