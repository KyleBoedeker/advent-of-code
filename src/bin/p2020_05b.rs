fn main() {
    // File hosts must exist in current path before this produces output

    // seats known to be occupied (not ours)
    let seats = include_str!("../../data/2020_p05_input.txt")
        .lines()
        .map(|seat_str| {
            seat_str
                .replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0")
        })
        .map(|bin_str| usize::from_str_radix(&bin_str, 2).unwrap())
        .map(|seat_num| (seat_num >> 3) * 8 + (seat_num & 0b111))
        .collect::<Vec<_>>();

    let (min, max) = (seats.iter().min().unwrap(), seats.iter().max().unwrap());
    println!("min: {}, max: {}", min, max);

    let mut possible_seats: Vec<_> = seats
        .iter()
        .filter(|s| !seats.contains(&(*s - 1)) || !seats.contains(&(*s + 1)))
        .collect();
    possible_seats.sort();

    println!("these are the seats with vacancies near them");
    println!("(the first/last elements are likely front/back of plane)");
    println!("{:?}", possible_seats);
}
