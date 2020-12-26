use std::collections::HashMap;

fn main() {
    let mut floating_bits = Vec::new();
    let mut floating_mask = !0;
    let mut overrides = 0;
    let mut memory = HashMap::new();
    String::from_utf8(std::fs::read("input/day14").unwrap())
        .unwrap()
        .split_terminator('\n')
        .for_each(|line| {
            let mut line = line.split(" = ");
            let left = line.next().unwrap();
            let right = line.next().unwrap();
            match left {
                "mask" => {
                    floating_bits = right
                        .chars()
                        .rev()
                        .enumerate()
                        .filter(|&(_, c)| c == 'X')
                        .map(|(i, _)| i as u8)
                        .collect();
                    floating_mask = u64::from_str_radix(
                        &right
                            .chars()
                            .map(|c| match c {
                                'X' => '0',
                                _ => '1',
                            })
                            .collect::<String>(),
                        2,
                    )
                    .unwrap();
                    overrides = u64::from_str_radix(
                        &right
                            .chars()
                            .map(|c| match c {
                                'X' => '0',
                                digit => digit,
                            })
                            .collect::<String>(),
                        2,
                    )
                    .unwrap();
                }
                _ => {
                    let idx = left
                        .strip_prefix("mem[")
                        .unwrap()
                        .strip_suffix("]")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        & floating_mask
                        | overrides;
                    let value = u64::from_str_radix(right, 10).unwrap();
                    for floating_value in 0..(1 << floating_bits.len()) {
                        let mut idx = idx;
                        for bit in 0..floating_bits.len() {
                            idx = idx | ((floating_value & 1 << bit) >> bit) << floating_bits[bit];
                        }
                        if value == 0 {
                            memory.remove(&idx);
                        } else {
                            *memory.entry(idx).or_insert(0) = value;
                        }
                    }
                }
            }
        });
    let answer: u64 = memory.values().into_iter().sum();
    println!("{}", answer);
}
