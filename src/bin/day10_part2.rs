fn main() {
    let mut input = String::from_utf8(std::fs::read("input/day10").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    let mut diff = [0u64; 3];
    diff[0] = 1;
    input
        .iter()
        .copied()
        .enumerate()
        .skip(1)
        .for_each(|(i, next)| {
            diff[i % diff.len()] = input[..i]
                .iter()
                .copied()
                .enumerate()
                .rev()
                .take_while(|&(_, p)| next - p <= 3)
                .map(|(j, _)| diff[j % diff.len()])
                .sum();
        });
    let answer = diff[(input.len() - 1) % diff.len()];
    println!("{}", answer);
}
