use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        println!("{}", process_line(&line).trim());
    }
}

fn process_line(line: &str) -> String {
    let mut output_str = String::new();
    let mut is_start_of_line = true;

    // Break up the line by whitespace
    let words: Vec<&str> = line.trim().split_whitespace().collect();

    for word in words.iter() {
        let new_word = match word.parse::<u8>() {
            Ok(num) => {
                if is_start_of_line {
                    capitalise_first_letter(&number_to_words(num))
                }
                else {
                    number_to_words(num)
                }
            }
            Err(_) => word.to_string()
        };

        output_str.push_str(&format!("{} ", &new_word));

        is_start_of_line = false;
    }

    output_str
}

fn capitalise_first_letter(word: &str) -> String {
    let mut characters = word.chars();
    match characters.next() {
        None => String::new(),
        Some(letter) => letter.to_uppercase().collect::<String>() + characters.as_str(),
    }
}

fn number_to_words(num: u8) -> String {
    // If its just 0, return zero
    if num == 0 {
        return String::from("zero");
    }

    let mut output_str = String::new();

    let tens_digit = num / 10;
    let units_digit = num % 10;

    if tens_digit > 1 {
        // Anything from 20 pretty much works the same
        match tens_digit {
            9 => output_str.push_str("ninety"),
            8 => output_str.push_str("eighty"),
            7 => output_str.push_str("seventy"),
            6 => output_str.push_str("sixty"),
            5 => output_str.push_str("fifty"),
            4 => output_str.push_str("forty"),
            3 => output_str.push_str("thirty"),
            2 => output_str.push_str("twenty"),
            _ => panic!("Should not happen")
        }

        // If the units digit is not 0, then append a hyphen and the digit itself
        if units_digit != 0 {
            output_str.push_str("-");
            output_str.push_str(&digit_to_unit(units_digit));
        }
    }
    else if tens_digit == 1 {
        // 10 to 19 are special cases
        match units_digit {
            9 => output_str.push_str("nineteen"),
            8 => output_str.push_str("eighteen"),
            7 => output_str.push_str("seventeen"),
            6 => output_str.push_str("sixteen"),
            5 => output_str.push_str("fifteen"),
            4 => output_str.push_str("fourteen"),
            3 => output_str.push_str("thirteen"),
            2 => output_str.push_str("twelve"),
            1 => output_str.push_str("eleven"),
            0 => output_str.push_str("ten"),
            _ => panic!("Should not happen")
        }
    }
    else {
        // Otherwise, its just a single-digit number
        output_str.push_str(&digit_to_unit(units_digit));
    }

    output_str
}

fn digit_to_unit(digit: u8) -> String {
    match digit {
        9 => String::from("nine"),
        8 => String::from("eight"),
        7 => String::from("seven"),
        6 => String::from("six"),
        5 => String::from("five"),
        4 => String::from("four"),
        3 => String::from("three"),
        2 => String::from("two"),
        1 => String::from("one"),
        _ => panic!("Should not happen")
    }
}
