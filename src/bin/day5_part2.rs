fn main() {
    let mut sits = String::from_utf8(std::fs::read("input/day5").unwrap())
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
        .collect::<Vec<_>>();
    sits.sort();
    let answer = sits
        .windows(2)
        .find(|neighbors| neighbors[1] - neighbors[0] == 2)
        .map(|neighbors| neighbors[0] + 1)
        .unwrap();
    println!("{}", answer);
}
