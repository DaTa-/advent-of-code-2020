use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let input = String::from_utf8(std::fs::read("input/day16").unwrap()).unwrap();
    let mut input = input.split("\n\n");

    let fields: Vec<(String, Vec<RangeInclusive<u16>>)> = input
        .next()
        .unwrap()
        .split_terminator('\n')
        .map(|field| {
            let mut field = field.split(": ");
            let name = field.next().unwrap().into();
            let ranges = field
                .next()
                .unwrap()
                .split(" or ")
                .map(|range| {
                    let mut range = range.split('-');
                    range.next().unwrap().parse().unwrap()..=range.next().unwrap().parse().unwrap()
                })
                .collect();
            (name, ranges)
        })
        .collect();

    fn parse_ticket(ticket: &str) -> Vec<u16> {
        ticket.split(',').map(|v| v.parse().unwrap()).collect()
    }

    let your_ticket = input
        .next()
        .unwrap()
        .split_terminator('\n')
        .skip(1)
        .map(parse_ticket)
        .next()
        .unwrap();
    let mut nearby_tickets: Vec<_> = input
        .next()
        .unwrap()
        .split_terminator('\n')
        .skip(1)
        .map(parse_ticket)
        .collect();
    nearby_tickets.retain(|ticket| {
        ticket.iter().all(|field| {
            fields
                .iter()
                .flat_map(|(_, ranges)| ranges.iter())
                .any(|range| range.contains(field))
        })
    });

    let mut possible_column_fields: Vec<HashSet<_>> = (0..fields.len())
        .map(|_| (0..fields.len()).collect())
        .collect();
    nearby_tickets
        .iter()
        .chain(std::iter::once(&your_ticket))
        .for_each(|ticket| {
            ticket.iter().enumerate().for_each(|(i, v)| {
                let possible_fields = &mut possible_column_fields[i];
                if possible_fields.len() != 1 {
                    possible_fields.retain(|&field| {
                        let ranges = &fields[field].1;
                        ranges.iter().any(|range| range.contains(v))
                    });
                }
            });
        });

    // exclude estimated guesses from other columns
    for i in 0..possible_column_fields.len() {
        let possible_fields = &possible_column_fields[i];
        if let Some(estimated_guess) = possible_fields
            .iter()
            .copied()
            .next()
            .filter(|_| possible_fields.len() == 1)
        {
            possible_column_fields
                .iter_mut()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .for_each(|(_, other_column_possible_fields)| {
                    other_column_possible_fields.remove(&estimated_guess);
                })
        }
    }

    loop {
        let mut modified = false;
        for i in 0..possible_column_fields.len() {
            let possible_fields = &possible_column_fields[i];
            if possible_fields.len() != 1 {
                if let Some(unique_field) = possible_fields.iter().copied().find(|&field| {
                    possible_column_fields
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .all(|(_, other_column_possible_fields)| {
                            !other_column_possible_fields.contains(&field)
                        })
                }) {
                    let possible_fields = &mut possible_column_fields[i];
                    possible_fields.clear();
                    possible_fields.insert(unique_field);
                    modified = true;
                }
            }
        }
        if !modified {
            break;
        }
    }

    assert!(possible_column_fields.iter().all(|p| p.len() == 1));
    let possible_column_fields: Vec<_> = possible_column_fields
        .into_iter()
        .map(|single_guess| single_guess.into_iter().next().unwrap())
        .collect();

    let answer: u64 = your_ticket
        .iter()
        .enumerate()
        .map(|(i, value)| (possible_column_fields[i], value))
        .filter(|&(i, _)| fields[i].0.starts_with("departure"))
        .map(|(_, value)| *value as u64)
        .product();
    println!("{}", answer);
}
