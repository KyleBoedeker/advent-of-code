#[derive(Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<&char> for Hand {
    fn from(hand_char: &char) -> Self {
        match hand_char {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Invalid character found!"),
        }
    }
}

enum GameResult {
    Win,
    Loss,
    Tie,
}

struct RockPaperScissorsGame {
    p1_hand: Hand,
    p2_hand: Hand,
}

impl From<&str> for RockPaperScissorsGame {
    fn from(line: &str) -> Self {
        // take in a string like "A X" and convert to "Rock vs Paper"
        let p2_hand = Hand::from(&line.chars().nth(0).unwrap());
        let p1_hand = Hand::from(&line.chars().nth(2).unwrap());
        return RockPaperScissorsGame { p1_hand, p2_hand };
    }
}

impl RockPaperScissorsGame {
    fn from_elf(line: &str) -> Self {
        let p2_hand = Hand::from(&line.chars().nth(0).unwrap());
        let p1_strat = &line.chars().nth(2).unwrap();
        let p1_hand = match p1_strat {
            'X' => {
                // lose
                match &p2_hand {
                    Hand::Rock => Hand::Scissors,
                    Hand::Paper => Hand::Rock,
                    Hand::Scissors => Hand::Paper,
                }
            }
            'Y' => {
                // tie
                p2_hand.clone()
            }
            'Z' => {
                // win
                match &p2_hand {
                    Hand::Rock => Hand::Paper,
                    Hand::Paper => Hand::Scissors,
                    Hand::Scissors => Hand::Rock,
                }
            }
            _ => panic!("Unknown Hand!"),
        };

        return RockPaperScissorsGame { p1_hand, p2_hand };
    }
}

impl RockPaperScissorsGame {
    fn get_result(&self) -> GameResult {
        match self.p1_hand {
            Hand::Rock => match self.p2_hand {
                Hand::Rock => GameResult::Tie,
                Hand::Paper => GameResult::Loss,
                Hand::Scissors => GameResult::Win,
            },
            Hand::Paper => match self.p2_hand {
                Hand::Rock => GameResult::Win,
                Hand::Paper => GameResult::Tie,
                Hand::Scissors => GameResult::Loss,
            },
            Hand::Scissors => match self.p2_hand {
                Hand::Rock => GameResult::Loss,
                Hand::Paper => GameResult::Win,
                Hand::Scissors => GameResult::Tie,
            },
        }
    }

    fn score_game(&self) -> usize {
        let p1_hand_value = match &self.p1_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        match self.get_result() {
            GameResult::Win => 6 + p1_hand_value,
            GameResult::Tie => 3 + p1_hand_value,
            GameResult::Loss => p1_hand_value,
        }
    }
}

fn main() {
    let input = include_str!("../../data/2022_p02_input.txt");

    let part_1_strat_score = input
        .lines()
        .map(|l| RockPaperScissorsGame::from(l).score_game())
        .sum::<usize>();

    println!("Assumed strategy score: {}", part_1_strat_score);

    let part_2_strat_score = input
        .lines()
        .map(|l| RockPaperScissorsGame::from_elf(l).score_game())
        .sum::<usize>();

    println!("Actual strategy score: {}", part_2_strat_score);
}
