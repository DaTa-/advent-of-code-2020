fn main() {
    const MAX_NUMBERS: usize = 2020;
    let input = "6,13,1,15,2,0"; // "0,3,6";
    let mut spoken_numbers = Vec::with_capacity(MAX_NUMBERS);
    spoken_numbers.extend(input.split(',').map(|n| n.parse::<u32>().unwrap()));
    for _ in 0..MAX_NUMBERS - spoken_numbers.len() {
        let last_num = *spoken_numbers.last().unwrap();
        spoken_numbers.push(
            spoken_numbers
                .iter()
                .rev()
                .skip(1)
                .position(|&n| n == last_num)
                .map(|diff| (diff + 1) as _)
                .unwrap_or(0),
        );
    }
    let answer = *spoken_numbers.last().unwrap();
    println!("{}", answer);
}
