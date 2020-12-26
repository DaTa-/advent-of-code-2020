fn main() {
    let answer = String::from_utf8(std::fs::read("input/day5").unwrap())
        .unwrap()
        .split_whitespace()
        .map(|sit_str| {
            sit_str
                .chars()
                .map(|c| match c {
                    'F' | 'L' => '0',
                    _ => '1',
                })
                .collect::<String>()
        })
        .map(|sit| u16::from_str_radix(&sit, 2).unwrap())
        .max()
        .unwrap();
    println!("{}", answer);
}
