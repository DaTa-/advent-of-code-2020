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
    while decks.iter().all(|deck| !deck.is_empty()) {
        let cards: Vec<_> = decks.iter_mut().map(|deck| deck.remove(0)).collect();
        let winner = cards
            .iter()
            .copied()
            .enumerate()
            .max_by_key(|&(_, card)| card)
            .map(|(player, _)| player)
            .unwrap();
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
    let won_deck = decks.iter().find(|deck| !deck.is_empty()).unwrap();
    let answer: u16 = won_deck
        .iter()
        .rev()
        .copied()
        .zip(1..)
        .map(|(card, score)| card as u16 * score as u16)
        .sum();
    println!("{}", answer);
}
