fn main() {
    let input = String::from_utf8(std::fs::read("input/day13").unwrap()).unwrap();
    let mut input = input.split_terminator("\n");
    let _ = input.next();
    let mut buses: Vec<(u64, u64)> = input
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(delay, bus)| {
            bus.parse()
                .ok()
                .map(|bus| ((bus - delay as u64 % bus) % bus, bus))
        })
        .collect();
    buses.sort_by_key(|&(_, bus)| bus);
    let (mut i, mut step) = buses.pop().unwrap();
    while let Some((rem, modulo)) = buses.pop() {
        loop {
            if i % modulo == rem {
                step *= modulo;
                break;
            }
            i += step;
        }
    }
    let answer = i;
    println!("{}", answer);
}
