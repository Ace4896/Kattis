use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases: u8 = input.trim().parse().unwrap();

    for _i in 0..test_cases {
        simon_says();
    }
}

fn simon_says() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.starts_with("simon says ") {
        let temp = input.split_off(11);
        let command = temp.trim();
        println!("{}", command);
    }
    else {
        println!("");
    }
}
