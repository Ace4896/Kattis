use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases: u8 = input.trim().parse().unwrap();
    
    for _ in 0..test_cases {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input_nums: Vec<u32> = input.split(' ').map(|x| x.trim().parse().unwrap()).collect();        
        let total = input_nums.iter().skip(1).fold(0u32, |sum, val| sum + val);
        let average: f64 = (total as f64) / (input_nums[0] as f64);
        
        let num_above_average: f64 = input_nums.iter().skip(1).fold(0f64, |count, avg| {
            if (*avg as f64) > average {
                count + 1f64
            }
            else {
                count
            }
        });
        
        let percent_above_average: f64 = num_above_average / (input_nums[0] as f64) * 100f64;
        println!("{:.3}%", percent_above_average);
    }
}
