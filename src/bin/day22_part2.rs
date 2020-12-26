use std::collections::HashSet;

fn main() {
    let mut decks: Vec<Vec<u8>> = String::from_utf8(std::fs::read("input/day22").unwrap())
        .unwrap()
        .split("\n\n")
        .map(|player| {
            player
                .split_terminator('\n')
                .skip(1)
                .map(|card| card.parse().unwrap())
                .collect()
        })
        .collect();

    fn play(decks: &mut [Vec<u8>]) -> usize {
        let mut previous_rounds = HashSet::<Vec<_>>::new();
        while decks.iter().all(|deck| !deck.is_empty()) {
            if !previous_rounds.insert(decks.to_vec()) {
                return 0;
            }

            let cards: Vec<_> = decks.iter_mut().map(|deck| deck.remove(0)).collect();
            let winner = if cards
                .iter()
                .copied()
                .zip(decks.iter().map(|deck| deck.len()))
                .all(|(card, cards_left)| cards_left >= card as usize)
            {
                let mut sub_decks: Vec<_> = decks
                    .iter()
                    .zip(&cards)
                    .map(|(deck, &card)| deck.iter().copied().take(card as usize).collect())
                    .collect();
                play(&mut sub_decks)
            } else {
                cards
                    .iter()
                    .copied()
                    .enumerate()
                    .max_by_key(|&(_, card)| card)
                    .map(|(player, _)| player)
                    .unwrap()
            };

            decks[winner].push(cards[winner]);
            decks[winner].extend(
                cards
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(|&(player, _)| player != winner)
                    .map(|(_, card)| card),
            );
        }
        decks.iter().position(|deck| !deck.is_empty()).unwrap()
    }

    let winner = play(&mut decks);
    let answer: u16 = decks[winner]
        .iter()
        .rev()
        .copied()
        .zip(1..)
        .map(|(card, score)| card as u16 * score as u16)
        .sum();
    println!("{}", answer);
}
