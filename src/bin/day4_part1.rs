fn main() {
    const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let answer = String::from_utf8(std::fs::read("input/day4").unwrap())
        .unwrap()
        .split("\n\n")
        .filter(|passport| {
            passport
                .split_whitespace()
                .map(|record| record.split(':').next().unwrap())
                .fold([false; REQUIRED_FIELDS.len()], |mut available, field| {
                    if let Some(field) = REQUIRED_FIELDS.iter().position(|&f| f == field) {
                        available[field] = true;
                    }
                    available
                })
                .iter()
                .all(|&present| present)
        })
        .count();
    println!("{}", answer);
}
