use std::collections::HashSet;

fn main() {
    let input = String::from_utf8(std::fs::read("input/day1").unwrap())
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.parse().unwrap())
        .collect::<HashSet<u32>>();
    const TARGET_SUM: u32 = 2020;
    let answer = input
        .iter()
        .copied()
        .flat_map(|a| {
            input
                .iter()
                .copied()
                .filter(move |&b| b != a)
                .map(move |b| [a, b])
        })
        .filter(|&[a, b]| a + b < TARGET_SUM)
        .map(|[a, b]| [a, b, TARGET_SUM - a - b])
        .find(|[_, _, c]| input.contains(c))
        .map(|[a, b, c]| a * b * c)
        .unwrap();
    println!("{}", answer);
}
