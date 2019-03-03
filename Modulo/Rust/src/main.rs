use std::io;

fn main() {
    let mut unique_nums: Vec<u16> = Vec::new();

    for _i in 0..10 {
        let modulo_num = get_number() % 42;
        if unique_nums.contains(&modulo_num) == false {
            unique_nums.push(modulo_num);
        }
    }

    println!("{}", unique_nums.len());
}

fn get_number() -> u16 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
