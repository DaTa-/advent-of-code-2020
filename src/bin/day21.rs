use std::collections::{HashMap, HashSet};

fn main() {
    let input = String::from_utf8(std::fs::read("input/day21").unwrap()).unwrap();
    let mut allergens = HashMap::<&str, Vec<usize>>::new();
    let mut food = Vec::<HashSet<&str>>::new();
    input.split_terminator('\n').for_each(|line| {
        let mut line = line.split(" (contains ");
        let possible_food: HashSet<_> = line.next().unwrap().split(' ').collect();
        let food_index = food.len();
        food.push(possible_food);
        line.next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .for_each(|allergen| {
                let allergen = allergens.entry(allergen).or_insert_with(|| Vec::new());
                allergen.push(food_index);
            });
    });
    let mut allergenic_food = HashMap::<&str, &str>::new();
    let mut processed_allergens = HashSet::<&str>::new();
    loop {
        let old_len = processed_allergens.len();

        for (&allergen, possible_food) in allergens.iter() {
            if processed_allergens.contains(&allergen) {
                continue;
            }

            let mut possible_food = possible_food.iter().map(|&food_index| &food[food_index]);
            let intersection: HashSet<_> = possible_food
                .next()
                .unwrap()
                .iter()
                .copied()
                .filter(|food| !allergenic_food.contains_key(food))
                .collect();
            let intersection = possible_food.fold(intersection, |intersection, possible_food| {
                intersection.intersection(possible_food).copied().collect()
            });

            if intersection.len() == 1 {
                allergenic_food.insert(intersection.into_iter().next().unwrap(), allergen);
                processed_allergens.insert(allergen);
            }
        }

        if processed_allergens.len() == allergens.len() {
            break;
        }
        assert_ne!(processed_allergens.len(), old_len);
    }

    let part1 = food
        .iter()
        .flat_map(|food| food.iter().copied())
        .filter(|&food| !allergenic_food.contains_key(&food))
        .count();
    println!("{}", part1);

    let mut part2: Vec<_> = allergenic_food.into_iter().collect();
    part2.sort_by_key(|&(_, allergen)| allergen);
    let part2 = part2
        .into_iter()
        .map(|(food, _)| food)
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", part2);
}
