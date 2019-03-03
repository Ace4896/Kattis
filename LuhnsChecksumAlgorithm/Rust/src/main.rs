use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let test_cases = input.trim().parse().unwrap();

    for _i in 0..test_cases {
        checksum();
    }
}

fn checksum() {
    const RADIX: u32 = 10;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut transform = false;
    let mut sum: u32 = 0;

    for c in input.trim().chars().rev() {
        match transform {
            true => {
                eprintln!("Current Digit (Transform = TRUE):  {}", c);
                let num: u32 = c.to_digit(RADIX).unwrap();
                let initial_num = num * 2;

                if initial_num > 9 {
                    let tens_digit = initial_num / 10;
                    let units_digit = initial_num % 10;
                    sum += tens_digit + units_digit;
                }
                else {
                    sum += initial_num;
                }

                transform = false;
            },
            false => {
                eprintln!("Current Digit (Transform = FALSE): {}", c);
                let num_to_add: u32 = c.to_digit(RADIX).unwrap();
                sum += num_to_add;
                transform = true;
            }
        }
    }

    if sum % 10 == 0 {
        println!("PASS");
    }
    else {
        println!("FAIL");
    }
}
