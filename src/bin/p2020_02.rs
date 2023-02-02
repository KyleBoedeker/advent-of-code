use std::fmt;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct ParseDBEntryError;

#[derive(Debug, PartialEq)]
struct DBEntry {
    password: String,
    letter: char,
    n1: usize,
    n2: usize,
}

impl DBEntry {
    fn from_str(s: &str) -> Result<Self, ParseDBEntryError> {
        let v: Vec<&str> = s.split(['-', ' ']).collect();

        let n1 = v[0].parse().map_err(|_| ParseDBEntryError)?;
        let n2 = v[1].parse().map_err(|_| ParseDBEntryError)?;
        let letter = v[2].chars().next().ok_or(ParseDBEntryError)?;
        let password = String::from(v[3]);

        return Ok(DBEntry {
            password,
            letter,
            n1,
            n2,
        });
    }

    fn is_valid_password_by_old_policy(&self) -> bool {
        let range = self.n1..=self.n2;
        return range.contains(&(self.password.matches(self.letter).count()));
    }

    fn is_valid_password_by_policy(&self) -> bool {
        // don't forget to subtract 1 so we zero index access the password
        let b1 = self.password.chars().nth(self.n1 - 1).unwrap() == self.letter;
        let b2 = self.password.chars().nth(self.n2 - 1).unwrap() == self.letter;
        // XOR (likely re-inventing the wheel)
        return (!b1 && b2) || (b1 && !b2);
    }
}

impl fmt::Display for DBEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}-{} {}: {}",
            self.n1, self.n2, self.letter, self.password
        );
    }
}

fn main() {
    let buffer = fs::read_to_string("data/2020_p02_input.txt").expect("Input file was not found");

    // parse lines as database entries and count the # of valid ones
    let valid_password_count = buffer
        .lines()
        .map(|line| DBEntry::from_str(line).expect("Could not parse line!"))
        .filter(|db| db.is_valid_password_by_old_policy())
        .count();

    println!(
        "# of valid passwords (by old policy): {}",
        valid_password_count
    );

    // parse lines as database entries and count the # of valid ones
    let valid_password_count = buffer
        .lines()
        .map(|line| DBEntry::from_str(line).expect("Could not parse line!"))
        .filter(|db| db.is_valid_password_by_policy())
        .count();

    println!("# of valid passwords: {}", valid_password_count);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_db() {
        let db_entry_str = "1-3 a: abcde";
        assert_eq!(
            DBEntry::from_str(db_entry_str),
            Ok(DBEntry {
                letter: 'a',
                n1: 1,
                n2: 3,
                password: String::from("abcde")
            })
        );
    }

    #[test]
    fn test_fail_create_db() {
        // invalid database entry (letter and max occurrences backwards)
        let db_entry_str = "1-a 3: abcde";
        assert!(DBEntry::from_str(db_entry_str).is_err());
    }

    #[test]
    fn test_is_valid_password_by_old_policy() {
        let valid_db_entries = vec!["1-3 a: abcde", "2-9 c: ccccccccc", "2-9 c: cc"];
        let invalid_db_entries = vec!["1-3 b: cdefg"];

        for valid_db_entry in valid_db_entries {
            assert!(DBEntry::from_str(valid_db_entry)
                .unwrap()
                .is_valid_password_by_old_policy());
        }
        for invalid_db_entry in invalid_db_entries {
            assert!(!DBEntry::from_str(invalid_db_entry)
                .unwrap()
                .is_valid_password_by_old_policy());
        }
    }

    #[test]
    fn test_is_valid_password_by_policy() {
        let valid_db_entries = vec!["1-3 a: abcde"];
        let invalid_db_entries = vec!["1-3 b: cdefg", "2-9 c: ccccccccc"];

        for valid_db_entry in valid_db_entries {
            assert!(DBEntry::from_str(valid_db_entry)
                .unwrap()
                .is_valid_password_by_policy());
        }
        for invalid_db_entry in invalid_db_entries {
            assert!(!DBEntry::from_str(invalid_db_entry)
                .unwrap()
                .is_valid_password_by_policy());
        }
    }
}
