fn main() {
    let input = String::from_utf8(std::fs::read("input/day13").unwrap()).unwrap();
    let mut input = input.split_terminator("\n");
    let timestamp: u32 = input.next().unwrap().parse().unwrap();
    let buses: Vec<u32> = input
        .next()
        .unwrap()
        .split(',')
        .filter_map(|bus| bus.parse().ok())
        .collect();
    let answer = buses
        .iter()
        .map(|bus| (bus, (bus - timestamp % bus) % bus))
        .min_by_key(|&(_, delay)| delay)
        .map(|(bus, delay)| bus * delay)
        .unwrap();
    println!("{}", answer);
}
