use std::collections::HashMap;

fn main() {
    const MAX_NUMBERS: u32 = 30000000;
    let input = "6,13,1,15,2,0"; // "0,3,6";

    let mut input = input.split(',').rev().map(|n| n.parse().unwrap());
    let mut last_num = input.next().unwrap();
    let input = input.rev();

    let mut spoken_count = 0;
    let mut spoken_numbers = HashMap::new();
    for n in input {
        spoken_numbers.insert(n, spoken_count);
        spoken_count += 1;
    }
    spoken_count += 1; // last number isn't mapped
    for i in spoken_count..MAX_NUMBERS {
        let last_num_idx = i - 1;
        last_num = spoken_numbers
            .insert(last_num, last_num_idx)
            .map_or(0, |prev| last_num_idx - prev);
    }

    let answer = last_num;
    println!("{}", answer);
}
