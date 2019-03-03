use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let data_sets: u32 = input.trim().parse().unwrap();

    for _i in 0..data_sets {
        calculate_dataset();
    }
}

fn calculate_dataset() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

    println!("{} {} {} {}",
        numbers[0],
        arithmetic_sequence_sum(numbers[1], 1, 1),
        arithmetic_sequence_sum(numbers[1], 1, 2),
        arithmetic_sequence_sum(numbers[1], 2, 2));
}

fn arithmetic_sequence_sum(n: u32, a: u32, d: u32) -> u32 {
    let numerator = n * (2 * a + (n - 1) * d);
    numerator / 2
}
