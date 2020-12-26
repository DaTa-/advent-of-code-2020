fn main() {
    const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let answer = String::from_utf8(std::fs::read("input/day4").unwrap())
        .unwrap()
        .split("\n\n")
        .filter(|passport| {
            passport
                .split_whitespace()
                .map(|record| {
                    let mut pair = record.split(':');
                    (pair.next().unwrap(), pair.next().unwrap())
                })
                .fold(
                    [false; REQUIRED_FIELDS.len()],
                    |mut valid_fields, (field, value)| {
                        let valid = match field {
                            "byr" => {
                                value.len() == 4
                                    && value
                                        .parse::<u16>()
                                        .map_or(false, |value| value >= 1920 && value <= 2002)
                            }
                            "iyr" => {
                                value.len() == 4
                                    && value
                                        .parse::<u16>()
                                        .map_or(false, |value| value >= 2010 && value <= 2020)
                            }
                            "eyr" => {
                                value.len() == 4
                                    && value
                                        .parse::<u16>()
                                        .map_or(false, |value| value >= 2020 && value <= 2030)
                            }
                            "hgt" => value.chars().position(|c| !c.is_ascii_digit()).map_or(
                                false,
                                |units_offset| match &value[units_offset..] {
                                    "cm" => value[..units_offset]
                                        .parse::<u8>()
                                        .map_or(false, |value| value >= 150 && value <= 193),

                                    "in" => value[..units_offset]
                                        .parse::<u8>()
                                        .map_or(false, |value| value >= 59 && value <= 76),
                                    _ => return false,
                                },
                            ),
                            "hcl" => {
                                value.chars().next().map_or(false, |c| c == '#')
                                    && value.chars().skip(1).all(|c| {
                                        c.is_ascii_digit()
                                            || c.is_ascii_hexdigit() && c.is_ascii_lowercase()
                                    })
                            }
                            "ecl" => {
                                const VALID_EYE_COLORS: &[&str] =
                                    &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                                VALID_EYE_COLORS.contains(&value)
                            }
                            "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
                            "cid" => true,
                            _ => unreachable!(),
                        };
                        if valid {
                            if let Some(field) = REQUIRED_FIELDS.iter().position(|&f| f == field) {
                                valid_fields[field] = true;
                            }
                        }
                        valid_fields
                    },
                )
                .iter()
                .all(|&present| present)
        })
        .count();
    println!("{}", answer);
}
