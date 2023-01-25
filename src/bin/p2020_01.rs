use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/2020_p01_input.txt").expect("Input file was not found");

    let reader = BufReader::new(input_file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    for (i1, n1) in numbers.iter().enumerate() {
        for (i2, n2) in numbers.iter().skip(i1).enumerate() {
            for n3 in numbers.iter().skip(i2) {
                if n1 + n2 + n3 == 2020 {
                    println!("{} + {} + {} = {}", n1, n2, n3, n1 + n2 + n3);
                    println!("{} * {} * {} = {}", n1, n2, n3, n1 * n2 * n3);
                }
            }
        }
    }
}
