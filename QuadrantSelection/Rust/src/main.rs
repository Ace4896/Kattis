use std::io;

fn main() {
    let x = read_input();
    let y = read_input();

    if y < 0 {
        if x < 0 {
            println!("3");
        }
        else {
            println!("4");
        }
    }
    else {
        if x < 0 {
            println!("2");
        }
        else {
            println!("1");
        }
    }


}

fn read_input() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Could not read line");

    input.trim().parse().unwrap()
}
