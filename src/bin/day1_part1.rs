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
        .map(|a| [a, TARGET_SUM - a])
        .find(|&[_, b]| input.contains(&b))
        .map(|[a, b]| a * b)
        .unwrap();
    println!("{}", answer);
}
