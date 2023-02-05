use std::collections::HashMap;

use mylib::read_lines;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from(fields: HashMap<String, String>) -> Self {
        return Self { fields };
    }

    fn is_valid(&self) -> bool {
        let reqd_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        return reqd_keys
            .iter()
            .all(|rk| self.fields.contains_key(&String::from(*rk)));
    }
}

fn count_valid_passports<'a>(vals: impl Iterator<Item = String>) -> usize {
    let mut passports = Vec::new();
    let mut fields = HashMap::new();

    for line in vals {
        if line.trim().is_empty() && !fields.is_empty() {
            passports.push(Passport::from(fields));
            fields = HashMap::new();
        }
        for kv_pair in line.trim().split_whitespace() {
            let mut pairing: Vec<String> = kv_pair.split(':').map(String::from).collect();
            assert_eq!(pairing.len(), 2);
            fields.insert(pairing.remove(0), pairing.remove(0));
        }
    }

    // don't forget the last passport in the file!
    if !fields.is_empty() {
        passports.push(Passport::from(fields));
    }

    return passports.iter().filter(|p| p.is_valid()).count();
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("data/2020_p04_input.txt") {
        println!(
            "num valid passports found: {}",
            count_valid_passports(lines.filter_map(|ln| ln.ok()))
        );
    }
    // 238 is too low
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let passport_string = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        assert_eq!(
            count_valid_passports(passport_string.lines().map(String::from)),
            2,
        )
    }

    #[test]
    fn test_last_pasport() {
        let passport_string = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm";

        assert_eq!(
            count_valid_passports(passport_string.lines().map(String::from)),
            1,
        )
    }
}
