fn main() {
    // File hosts must exist in current path before this produces output
    println!(
        "{}",
        include_str!("../../data/2020_p05_input.txt")
            .lines()
            .map(|seat_str| seat_str
                .replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0"))
            .map(|bin_str| usize::from_str_radix(&bin_str, 2).unwrap())
            .map(|seat_num| (seat_num >> 3) * 8 + (seat_num & 0b111))
            .max()
            .unwrap_or_default()
    );
}
