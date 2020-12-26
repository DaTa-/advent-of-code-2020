fn main() {
    let mut input = String::from_utf8(std::fs::read("input/day10").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    let mut ones = 0usize;
    let mut threes = 0;
    input.windows(2).for_each(|n| match n[1] - n[0] {
        1 => ones += 1,
        3 => threes += 1,
        _ => (),
    });
    let answer = ones * threes;
    println!("{}", answer);
}
