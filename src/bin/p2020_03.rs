use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct TobogganTrip {
    x_vel: usize,
    y_vel: usize, // 1 = every row, 2 = every other row, etc.
    x_pos: usize, // used for indexing rows
    y_pos: usize, // used for skipping rows
    first_update_called: bool, // used to consume first row
    trees_hit: usize,
}

impl TobogganTrip {
    fn new(x_vel: usize, y_vel: usize) -> Self {
        Self {
            x_vel,
            y_vel,
            y_pos: 1,
            x_pos: 0,
            first_update_called: false,
            trees_hit: 0,
        }
    }

    fn update(&mut self, forest_row: &str) {

        let mut skip_row = false;

        // move to new position (don't move if we're on the first row)
        if self.first_update_called {
            // is y-position still less than y-velocity? skip row if so
            if self.y_pos < self.y_vel {
                self.y_pos += 1;
                skip_row = true;
            }
            // update x-position based on whether we ought to consume this row
            if !skip_row {
                self.y_pos = 1;
                self.x_pos = (self.x_pos + self.x_vel) % forest_row.len();
            }
        } else {
            // unset the sticky bit (we've called update at least once)
            self.first_update_called = true;
        }

        // check whether the new position is a tree and update trees_hit
        if !skip_row {
            if forest_row.chars().nth(self.x_pos) == Some('#') {
                self.trees_hit += 1;
            }
        }

    }
}

fn main() {
    let mut trips = Vec::new();
    trips.push(TobogganTrip::new(1, 1));
    trips.push(TobogganTrip::new(3, 1));
    trips.push(TobogganTrip::new(5, 1));
    trips.push(TobogganTrip::new(7, 1));
    trips.push(TobogganTrip::new(1, 2));

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("data/2020_p03_input.txt") {
        for line in lines {
            if let Ok(forest_row) = line {
                for trip in trips.iter_mut() {
                    trip.update(&forest_row);
                }
            }
        }
    }

    // sift through here for part 1's answer
    for trip in trips.iter() {
        println!("{:?}", trip);
    }

    println!("{}", trips.iter().map(|t| t.trees_hit).product::<usize>());
    // goldylocks! 7560370818 (for part 2)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part2() {
        let mut trips = Vec::new();
        trips.push(TobogganTrip::new(1, 1));
        trips.push(TobogganTrip::new(3, 1));
        trips.push(TobogganTrip::new(5, 1));
        trips.push(TobogganTrip::new(7, 1));
        trips.push(TobogganTrip::new(1, 2));

        let map = "..##.......
                   #...#...#..
                   .#....#..#.
                   ..#.#...#.#
                   .#...##..#.
                   ..#.##.....
                   .#.#.#....#
                   .#........#
                   #.##...#...
                   #...##....#
                   .#..#...#.#";

        for line in map.lines().map(|l| l.trim()) {
            for trip in trips.iter_mut() {
                trip.update(line);
            }
        }

        assert_eq!(
            trips.iter().map(|t| t.trees_hit).collect::<Vec<usize>>(),
            vec![2, 7, 3, 4, 2]
        )
    }
}

//
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#