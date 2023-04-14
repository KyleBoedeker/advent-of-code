fn main() {
    let mut numbers: Vec<usize> = include_str!("../../data/2022_p01_input.txt")
        .split("\n\n")
        .map(|elf_inventory| {
            elf_inventory
                .split_ascii_whitespace()
                .map(|l| l.trim().parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();

    numbers.sort();

    println!(
        "Top elf calorie count: {}",
        numbers.iter().rev().next().unwrap()
    );

    println!(
        "Top 3 elf calorie total: {}",
        numbers.iter().rev().take(3).sum::<usize>()
    );
}
