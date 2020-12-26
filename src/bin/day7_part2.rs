use std::collections::HashMap;

fn main() {
    let all_bags = String::from_utf8(std::fs::read("input/day7").unwrap())
        .unwrap()
        .split("\n")
        .filter(|rule| !rule.is_empty())
        .map(|rule| {
            const SEPARATOR: &str = " bags contain ";
            let color_end = rule.find(SEPARATOR).unwrap();
            let color = rule[..color_end].to_owned();
            let tail = &rule[color_end..][SEPARATOR.len()..];
            let inner_bags = if tail == "no other bags." {
                HashMap::new()
            } else {
                tail.strip_suffix(".")
                    .unwrap()
                    .split(", ")
                    .map(|inner_bag| {
                        let mut inner_bag = inner_bag.split(" ");
                        let (quantity, color) = (
                            inner_bag.next().unwrap(),
                            format!(
                                "{} {}",
                                inner_bag.next().unwrap(),
                                inner_bag.next().unwrap()
                            ),
                        );
                        (color, quantity.parse().unwrap())
                    })
                    .collect()
            };
            (color, inner_bags)
        })
        .collect::<HashMap<_, _>>();
    let mut contained_bags = HashMap::new();
    contained_bags.insert("shiny gold".to_owned(), 1);
    let mut bags_counter = 0;
    while !contained_bags.is_empty() {
        bags_counter += contained_bags.values().sum::<usize>();
        let inner_bags = contained_bags
            .iter()
            .flat_map(|(color, quantity)| {
                all_bags[color]
                    .iter()
                    .map(move |(inner_color, inner_quantity)| {
                        (inner_color.clone(), quantity * inner_quantity)
                    })
            })
            .collect::<Vec<_>>();
        contained_bags.clear();
        inner_bags
            .into_iter()
            .for_each(|(color, quantity)| *contained_bags.entry(color).or_insert(0) += quantity);
    }
    let answer = bags_counter - 1;
    println!("{}", answer);
}
