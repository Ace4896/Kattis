use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Could not read line");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if n % 2 == 0 {
        println!("Bob");
    }
    else {
        println!("Alice");
    }
}
