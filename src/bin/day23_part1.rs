use std::char;

fn main() {
    let _example = (b"389125467", 10); // 92658374
    let _example2 = (b"389125467", 100); // 67384529
    let _task = (b"583976241", 100);

    let (input, cycles) = _task;
    let mut cups: Vec<_> = input.iter().map(|c| c - b'0' - 1).collect();
    assert!(cups.len() >= 5);

    let mut current_cup = 0;
    for _ in 0..cycles {
        let moved_cups = {
            const CUPS_TO_MOVE: usize = 3;
            let moved_cups_wrapped = (current_cup + 1)..(current_cup + 1 + CUPS_TO_MOVE);
            let moved_tail = moved_cups_wrapped.end - moved_cups_wrapped.end.min(cups.len());
            cups.rotate_left(moved_tail);
            current_cup -= moved_tail;
            moved_cups_wrapped.start - moved_tail..moved_cups_wrapped.end - moved_tail
        };

        let destination = (1..)
            .map(|i| ((cups[current_cup] as usize + cups.len() - i) % cups.len()) as u8)
            .map(|prev_cup_id| cups.iter().position(|&cup| cup == prev_cup_id).unwrap())
            .find(|&prev_cup| !moved_cups.contains(&prev_cup))
            .unwrap();
        if destination < moved_cups.start {
            cups[destination + 1..moved_cups.end].rotate_right(moved_cups.len());
        } else {
            cups[moved_cups.start..destination + 1].rotate_left(moved_cups.len());
        }
        if destination < current_cup {
            current_cup += moved_cups.len();
        }
        current_cup = (current_cup + 1) % cups.len();
    }
    let primary_position = cups.iter().position(|&cup| cup == 0).unwrap();
    let answer: String = (1..cups.len())
        .map(|offset| cups[(primary_position + offset) % cups.len()])
        .map(|cup| char::from_digit(cup as u32 + 1, 10).unwrap())
        .collect();
    println!("{}", answer);
}
