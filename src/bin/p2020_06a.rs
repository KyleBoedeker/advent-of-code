use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        include_str!("../../data/2020_p06_input.txt")
            .split("\n\n")
            .map(|answer_grp| answer_grp
                .chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<HashSet<char>>()
                .len())
            .sum::<usize>(),
    );
}
