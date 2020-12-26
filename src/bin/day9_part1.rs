fn main() {
    let preamble_len = 25;
    let mut input = String::from_utf8(std::fs::read("input/day9").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    let (buf, input) = input.split_at_mut(preamble_len);
    let answer = input
        .iter()
        .enumerate()
        .find(|&(i, &n)| {
            if buf
                .iter()
                .enumerate()
                .all(|(i, &a)| buf.iter().skip(i).all(|&b| a + b != n))
            {
                true
            } else {
                buf[i % buf.len()] = n;
                false
            }
        })
        .map(|(_, &n)| n)
        .unwrap();
    println!("{}", answer);
}
