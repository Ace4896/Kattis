use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    for i in 1..num+1 {
        println!("{} Abracadabra", i);
    }
}
