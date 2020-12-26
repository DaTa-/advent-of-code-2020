fn main() {
    let preamble_len = 25;
    let mut all_input = String::from_utf8(std::fs::read("input/day9").unwrap())
        .unwrap()
        .split_terminator("\n")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();
    let (buf, input) = all_input.split_at_mut(preamble_len);
    let mut buf = buf.to_vec();
    let buf = &mut buf[..];
    let invalid_n = input
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
    let sum_range = (0..all_input.len())
        .find_map(|i| {
            match all_input[i..]
                .iter()
                .enumerate()
                .try_fold(0, |mut sum, (i, &next)| {
                    sum += next;
                    if sum < invalid_n {
                        Ok(sum)
                    } else {
                        Err(Some(i + 1).filter(|_| sum == invalid_n))
                    }
                }) {
                Ok(_) | Err(None) => None,
                Err(Some(j)) => Some(&all_input[i..][..j]).filter(|range| range.len() >= 2),
            }
        })
        .unwrap();
    let min = sum_range.iter().copied().min().unwrap();
    let max = sum_range.iter().copied().max().unwrap();
    let answer = min + max;
    println!("{}", answer);
}
