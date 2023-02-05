use std::collections::HashMap;

const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const HCL_CHARS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    // File hosts must exist in current path before this produces output
    println!(
        "{}",
        include_str!("../../data/2020_p04_input.txt")
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split_once(":").unwrap())
                .collect::<HashMap<_, _>>())
            .filter(|passport| REQ_FIELDS.iter().all(|item| passport.contains_key(item)))
            .filter(|passport| passport.iter().all(|(f, v)| is_valid_passport_field(f, v)))
            .count(),
    );
}

fn is_valid_passport_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => value
            .parse::<usize>()
            .map_or_else(|_| false, |y| 1920 <= y && y <= 2002),
        "iyr" => value
            .parse::<usize>()
            .map_or_else(|_| false, |y| 2010 <= y && y <= 2020),
        "eyr" => value
            .parse::<usize>()
            .map_or_else(|_| false, |y| 2020 <= y && y <= 2030),
        "hgt" => {
            if let Ok(val) = value.trim_end_matches("cm").parse::<usize>() {
                return 150 <= val && val <= 193;
            }
            if let Ok(val) = value.trim_end_matches("in").parse::<usize>() {
                return 59 <= val && val <= 76;
            }
            false
        }
        "hcl" => value.starts_with('#') && value.chars().skip(1).all(|c| c.is_ascii_hexdigit()),
        "ecl" => HCL_CHARS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => panic!("uknown data entered"),
    }
}
