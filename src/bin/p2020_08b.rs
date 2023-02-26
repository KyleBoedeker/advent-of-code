use std::collections::HashSet;

fn main() {
    let program: Vec<&str> = include_str!("../../data/2020_p08_input.txt")
        .lines()
        .map(|l| l.trim())
        .collect();


    'outer: for (idx_inverted, line) in program.iter().enumerate() {
        if !(line.starts_with("nop") || line.starts_with("jmp")) {
            continue;
        }
        
        let mut visited_lines: HashSet<isize> = HashSet::new();

        let mut acc: isize = 0;
        let mut program_counter: isize = 0;

        loop {
            let is_flipped_nop_jmp = idx_inverted == program_counter as usize;

            let instruction = program[program_counter as usize];

            visited_lines.insert(program_counter);

            let (opcode, val) = instruction.split_once(' ').unwrap();

            if (opcode == "nop" && !is_flipped_nop_jmp) || (opcode == "jmp" && is_flipped_nop_jmp) {
                program_counter += 1;
                continue;
            }

            let val: isize = val.parse().unwrap();

            if opcode == "acc" {
                acc += val;
                program_counter += 1;
            } else if (opcode == "jmp" && !is_flipped_nop_jmp)
                || (opcode == "nop" && is_flipped_nop_jmp)
            {
                program_counter += val;
            }

            if visited_lines.contains(&program_counter) {
                break; // program is stuck in a loop (try next "swap" of nop and jmp)
            }

            if program_counter as usize >= program.len() {
                println!("Corrupted line is: line # {} '{}'", idx_inverted, line);
                println!("Accumulator is: {}", acc);
                break 'outer;
            }
        }
    }

    
}
