use std::collections::HashMap;

fn main() {
    let mut mask = !0;
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
                    mask = u64::from_str_radix(
                        &right
                            .chars()
                            .map(|c| match c {
                                'X' => '1',
                                _ => '0',
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
                        .unwrap();
                    let value = u64::from_str_radix(right, 10).unwrap();
                    let new_value = value & mask | overrides;
                    if new_value == 0 {
                        memory.remove(&idx);
                    } else {
                        *memory.entry(idx).or_insert(0) = value & mask | overrides;
                    }
                }
            }
        });
    let answer: u64 = memory.values().into_iter().sum();
    println!("{}", answer);
}
