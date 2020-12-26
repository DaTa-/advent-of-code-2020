use std::{iter, mem};

fn main() {
    let _example = (b"389125467", 10, 9); // 92658374
    let _example2 = (b"389125467", 100, 9); // 67384529
    let _task = (b"583976241", 10_000_000, 1_000_000);

    let (input, cycles, cups_count) = _task;
    let mut next_cup: Vec<_> = (0..cups_count).map(|cup| (cup + 1) % cups_count).collect();
    let input: Vec<_> = input.iter().map(|c| (c - b'0' - 1) as _).collect();
    input.windows(2).for_each(|neighbors| {
        next_cup[neighbors[0]] = neighbors[1];
    });
    if input.len() < next_cup.len() {
        *next_cup.last_mut().unwrap() = input[0];
        next_cup[*input.last().unwrap()] = input.len();
    } else {
        next_cup[*input.last().unwrap()] = input[0];
    }

    let mut current_cup = input[0];
    for _ in 0..cycles {
        let last_moved_cup = next_cup[next_cup[next_cup[current_cup]]];

        let destination = (1..)
            .map(|i| ((current_cup + next_cup.len() - i) % next_cup.len()))
            .find(|&prev_cup_id| {
                let mut prev_cup = Some(current_cup);
                iter::from_fn(|| {
                    prev_cup.take().map(|pc| {
                        let result = next_cup[pc];
                        if result != last_moved_cup {
                            prev_cup = Some(result);
                        }
                        result
                    })
                })
                .all(|moved_cup| moved_cup != prev_cup_id)
            })
            .unwrap();

        let after_moved_cup = next_cup[last_moved_cup];
        let first_moved_cup = mem::replace(&mut next_cup[current_cup], after_moved_cup);
        let after_destination = mem::replace(&mut next_cup[destination], first_moved_cup);
        next_cup[last_moved_cup] = after_destination;

        current_cup = next_cup[current_cup];
    }

    let a = next_cup[0];
    let b = next_cup[a];
    let answer = (a as u64 + 1) * (b as u64 + 1);
    println!("{}", answer);
}
