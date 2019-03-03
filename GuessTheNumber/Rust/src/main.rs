/*
Guess the number (1-1000).
- Essentially a binary search.
  - Choose middle = (upper_bound + lower_bound) / 2
    - If too low, set lower_bound = middle + 1
    - If too high, set upper_bound = middle - 1
    - Otherwise, answer obtained
*/

fn main() {
    let mut input = String::new();
    let mut lower_bound = 1;
    let mut upper_bound = 1000;

    loop {
        let middle = (upper_bound + lower_bound) / 2;
        println!("{}", middle);

        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().as_ref() {
            "lower" => upper_bound = middle - 1,
            "higher" => lower_bound = middle + 1,
            "correct" => return,
            _ => panic!("Shouldn't happen")
        }

        input.clear();
    }
}
