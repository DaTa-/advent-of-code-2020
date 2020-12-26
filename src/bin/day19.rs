use std::{collections::HashMap, str::FromStr};

enum Rule {
    Ref(usize),
    Literal(String),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(rule: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(literal) = rule.strip_prefix('"') {
            Rule::Literal(literal.strip_suffix('"').unwrap().into())
        } else {
            Rule::Ref(rule.parse().unwrap())
        })
    }
}

fn main() {
    let inputs = ["input/day19", "input/day19_2"];
    for input in &inputs {
        let input = String::from_utf8(std::fs::read(input).unwrap()).unwrap();
        let mut input = input.split("\n\n");
        let rules: HashMap<usize, Vec<Vec<Rule>>> = input
            .next()
            .unwrap()
            .split('\n')
            .map(|rule| {
                let mut rule = rule.split(": ");
                let n = rule.next().unwrap().parse().unwrap();
                let rules = rule
                    .next()
                    .unwrap()
                    .split(" | ")
                    .map(|rules| rules.split(' ').map(|rule| rule.parse().unwrap()).collect())
                    .collect();
                (n, rules)
            })
            .collect();

        fn matches_subrule<'text>(
            text: &'text str,
            subrule: &[Rule],
            all_rules: &HashMap<usize, Vec<Vec<Rule>>>,
        ) -> Vec<&'text str> {
            subrule
                .iter()
                .try_fold(vec![text], |tails, rule| {
                    Some(match rule {
                        Rule::Ref(rule) => tails
                            .into_iter()
                            .flat_map(|tail| matches_rule(tail, *rule, all_rules).into_iter())
                            .collect(),
                        Rule::Literal(lit) => tails
                            .into_iter()
                            .filter_map(|tail| tail.strip_prefix(lit))
                            .collect(),
                    })
                    .filter(|tails: &Vec<_>| !tails.is_empty())
                })
                .unwrap_or(Vec::new())
        }

        fn matches_rule<'text>(
            text: &'text str,
            rule: usize,
            all_rules: &HashMap<usize, Vec<Vec<Rule>>>,
        ) -> Vec<&'text str> {
            let rule = &all_rules[&rule];
            rule.iter()
                .flat_map(|rule| matches_subrule(text, rule, all_rules))
                .collect()
        }

        fn fully_matches_rule(
            text: &str,
            rule: usize,
            all_rules: &HashMap<usize, Vec<Vec<Rule>>>,
        ) -> bool {
            matches_rule(text, rule, all_rules)
                .into_iter()
                .any(|tail| tail.is_empty())
        }

        let messages = input.next().unwrap().split_terminator('\n');

        const PRIMARY_RULE: usize = 0;
        let answer = messages
            .filter(|&message| fully_matches_rule(message, PRIMARY_RULE, &rules))
            .count();
        println!("{}", answer);
    }
}
