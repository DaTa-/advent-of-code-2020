fn main() {
    let answer = String::from_utf8(std::fs::read("input/day6").unwrap())
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<std::collections::HashSet<_>>()
                .len()
        })
        .sum::<usize>();
    println!("{}", answer);
}
