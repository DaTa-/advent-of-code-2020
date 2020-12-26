fn main() {
    let input = String::from_utf8(std::fs::read("input/day2").unwrap()).unwrap();
    let input: Vec<_> = input
        .split_terminator('\n')
        .map(|s| {
            let mut split = s.split(": ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(policy, password)| {
            let mut policy = policy.split(' ');
            let seq = {
                let mut seq = policy.next().unwrap().split('-');
                [
                    seq.next().unwrap().parse().unwrap(),
                    seq.next().unwrap().parse().unwrap(),
                ]
            };
            assert!(seq[0] <= seq[1], "{:?}", seq);
            let letter = policy
                .next()
                .map(|c| {
                    assert_eq!(c.len(), 1);
                    c.chars().next().unwrap()
                })
                .unwrap();
            ((seq, letter), password)
        })
        .collect();

    let part1 = input
        .iter()
        .copied()
        .filter(|&((range, letter), password)| {
            let matches = password.chars().filter(|&c| c == letter).count();
            matches >= range[0] && matches <= range[1]
        })
        .count();
    println!("{}", part1);

    let part2 = input
        .iter()
        .copied()
        .filter(|&((seq, letter), password)| {
            seq.iter()
                .map(|&i| password.chars().nth(i - 1).unwrap())
                .filter(|&c| c == letter)
                .count()
                == 1
        })
        .count();
    println!("{}", part2);
}
