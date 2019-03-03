fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let names = input.trim().split('-');
    let mut output_name = String::new();

    for name in names {
        output_name.push(name.chars().next().unwrap());
    }

    println!("{}", output_name);
}
