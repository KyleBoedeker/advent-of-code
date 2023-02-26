use std::collections::HashSet;

fn main() {
    println!(
        "{:?}",
        include_str!("../../data/2020_p06_input.txt")
            .split("\n\n")
            .map(|answer_grp| answer_grp
                .split_ascii_whitespace()
                .map(|answer_usr| answer_usr.chars().collect::<HashSet<char>>())
                .reduce(|common, v| { common.intersection(&v).map(|c| *c).collect() })
                .unwrap()
                .len())
            // .collect::<Vec<usize>>() // for debugging
            .sum::<usize>(),
    );
}
