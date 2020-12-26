fn main() {
    let answer = String::from_utf8(std::fs::read("input/day6").unwrap())
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|person| person.chars().collect())
                .fold(
                    None,
                    |intersection: Option<std::collections::HashSet<_>>, person| match intersection
                    {
                        Some(prev) => Some(prev.intersection(&person).copied().collect()),
                        None => Some(person),
                    },
                )
                .unwrap()
                .len()
        })
        .sum::<usize>();
    println!("{}", answer);
}
