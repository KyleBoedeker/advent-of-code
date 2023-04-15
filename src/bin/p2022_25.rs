pub struct SNAFU {
    value: i64,
}

impl From<&str> for SNAFU {
    fn from(s: &str) -> Self {
        let mut value = 0;
        let mut place_value = 1;

        for c in s.chars().rev() {
            value += place_value
                * match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("Invalid SNAFU digit!"),
                };
            place_value *= 5;
        }

        SNAFU { value }
    }
}

impl SNAFU {
    fn num_snafu_digits(&self) -> u32 {
        let mut ndigits = 1;
        let mut base = 5;
        while (base / 2) + 1 <= self.value {
            base *= 5;
            ndigits += 1;
        }
        ndigits
    }
}

impl ToString for SNAFU {
    fn to_string(&self) -> String {
        let ndigits = self.num_snafu_digits();
        let mut base = 5i64.pow(ndigits - 1);
        let mut digits = vec![];

        let mut acc = if self.value > base * 3 / 2 {
            digits.push('2');
            2 * base
        } else {
            digits.push('1');
            base
        };

        if ndigits == 1 {
            return digits.into_iter().collect();
        }

        while base > 1 {
            base /= 5;

            let digit_selecter = [
                (acc - 2 * base).abs_diff(self.value),
                (acc - base).abs_diff(self.value),
                (acc).abs_diff(self.value),
                (acc + 1 * base).abs_diff(self.value),
                (acc + 2 * base).abs_diff(self.value),
            ];

            let min = digit_selecter.iter().min().unwrap();

            if *min == digit_selecter[0] {
                acc -= 2 * base;
                digits.push('=');
            } else if *min == digit_selecter[1] {
                acc -= base;
                digits.push('-');
            } else if *min == digit_selecter[2] {
                digits.push('0');
            } else if *min == digit_selecter[3] {
                acc += base;
                digits.push('1');
            } else if *min == digit_selecter[4] {
                acc += 2 * base;
                digits.push('2');
            }
        }
        digits.into_iter().collect()
    }
}

fn main() {
    let input = include_str!("../../data/2022_p25_input.txt");

    let total_fuel = input.lines().map(|l| SNAFU::from(l).value).sum::<i64>();

    println!("total fuel (decimal) for p1 is: '{}'", total_fuel);
    println!(
        "total fuel (snafu) for p1 is: '{}'",
        SNAFU { value: total_fuel }.to_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_from_str() {
        assert_eq!(SNAFU::from("1-2").value, 22);
        assert_eq!(SNAFU::from("12111").value, 906);
        assert_eq!(SNAFU::from("1=-0-2").value, 1747);
    }

    #[test]
    fn test_num_snafu_digits() {
        assert_eq!(SNAFU { value: 2 }.num_snafu_digits(), 1);
        assert_eq!(SNAFU { value: 3 }.num_snafu_digits(), 2);
        assert_eq!(SNAFU { value: 12 }.num_snafu_digits(), 2);
        assert_eq!(SNAFU { value: 13 }.num_snafu_digits(), 3);
        assert_eq!(SNAFU { value: 1747 }.num_snafu_digits(), 6);
        assert_eq!(SNAFU { value: 314159265 }.num_snafu_digits(), 13);
    }
    #[test]
    fn test_snafu_into_string() {
        assert_eq!(SNAFU { value: 2 }.to_string(), "2");
        assert_eq!(SNAFU { value: 3 }.to_string(), "1=");
        assert_eq!(SNAFU { value: 12 }.to_string(), "22");
        assert_eq!(SNAFU { value: 13 }.to_string(), "1==");
        assert_eq!(&SNAFU { value: 1747 }.to_string(), "1=-0-2");
        assert_eq!(&SNAFU { value: 906 }.to_string(), "12111");
        assert_eq!(&SNAFU { value: 4890 }.to_string(), "2=-1=0");
    }
}
