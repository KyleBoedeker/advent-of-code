use std::collections::HashSet;

fn main() {
    let program: Vec<&str> = include_str!("../../data/2020_p08_input.txt")
        .lines()
        .map(|l| l.trim())
        .collect();

    let mut visited_lines: HashSet<isize> = HashSet::new();

    let mut acc: isize = 0;
    let mut program_counter: isize = 0;

    loop {
        if visited_lines.contains(&program_counter) {
            break;
        }
        let instruction = program[program_counter as usize];

        visited_lines.insert(program_counter);

        let (opcode, val) = instruction.split_once(' ').unwrap();

        if opcode == "nop" {
            program_counter += 1;
            continue;
        }

        let val: isize = val.parse().unwrap();

        if opcode == "acc" {
            acc += val;
            program_counter += 1;
        } else if opcode == "jmp" {
            program_counter += val;
        }
    }

    println!("Accumulator is: {}", acc);
}
