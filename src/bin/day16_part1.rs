use std::ops::RangeInclusive;

fn main() {
    let input = String::from_utf8(std::fs::read("input/day16").unwrap()).unwrap();
    let mut input = input.split("\n\n");

    let fields: Vec<RangeInclusive<u16>> = input
        .next()
        .unwrap()
        .split_terminator('\n')
        .flat_map(|field| {
            let mut field = field.split(": ");
            let _name = field.next().unwrap();
            let ranges = field.next().unwrap();
            ranges.split(" or ").map(|range| {
                let mut range = range.split('-');
                range.next().unwrap().parse().unwrap()..=range.next().unwrap().parse().unwrap()
            })
        })
        .collect();

    let _your_ticket = input.next().unwrap();

    let answer: u16 = input
        .next()
        .unwrap()
        .split_terminator('\n')
        .skip(1)
        .flat_map(|ticket| {
            ticket
                .split(',')
                .map(|v| v.parse().unwrap())
                .filter(|v| fields.iter().all(|range| !range.contains(v)))
        })
        .sum();
    println!("{}", answer);
}
