fn main() {
    let x = get_number();
    let n = get_number();

    let mut usable = x;

    for _i in 0..n {
        usable -= get_number();
        usable += x;
    }

    println!("{}", usable);
}

fn get_number() -> u32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
