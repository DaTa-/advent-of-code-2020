use std::collections::{HashMap, HashSet};

fn main() {
    let mut all_bags = String::from_utf8(std::fs::read("input/day7").unwrap())
        .unwrap()
        .split("\n")
        .filter(|rule| !rule.is_empty())
        .map(|rule| {
            const SEPARATOR: &str = " bags contain ";
            let color_end = rule.find(SEPARATOR).unwrap();
            let color = rule[..color_end].to_owned();
            let tail = &rule[color_end..][SEPARATOR.len()..];
            let inner_bags = if tail == "no other bags." {
                HashSet::new()
            } else {
                tail.strip_suffix(".")
                    .unwrap()
                    .split(", ")
                    .map(|inner_bag| {
                        let mut inner_bag = inner_bag.split(" ");
                        let (_quantity, color) = (
                            inner_bag.next().unwrap(),
                            format!(
                                "{} {}",
                                inner_bag.next().unwrap(),
                                inner_bag.next().unwrap()
                            ),
                        );
                        color
                    })
                    .collect()
            };
            (color, inner_bags)
        })
        .collect::<HashMap<_, _>>();
    let mut allowed_bags = HashSet::new();
    allowed_bags.insert("shiny gold".to_owned());
    all_bags.retain(|outer, _| !allowed_bags.contains(outer));
    loop {
        let suitable_bags = all_bags
            .iter()
            .filter(|&(_, inner)| inner.intersection(&allowed_bags).next().is_some())
            .map(|(outer, _)| outer.clone())
            .collect::<HashSet<_>>();
        if suitable_bags.is_empty() {
            break;
        }
        all_bags.retain(|outer, _| !suitable_bags.contains(outer));
        allowed_bags.extend(suitable_bags);
    }
    let answer = allowed_bags.len() - 1;
    println!("{}", answer);
}
