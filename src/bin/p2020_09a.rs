
struct XMASWindow {
    values: [usize; 25],
    index_to_replace: usize,
    wrapped: bool,
}

impl XMASWindow {
    pub fn new() -> Self {
        return XMASWindow { index_to_replace: 0, values: [0;25], wrapped: false };
    }

    pub fn roll_with(&mut self, v: &usize) -> bool {
        // returns true if value part of "preamble" or is producible from sum of 2 different preamble #'s
        if !self.wrapped {
            self.values[self.index_to_replace] = *v;
            self.index_to_replace = (self.index_to_replace + 1) % 25;
            self.wrapped = self.index_to_replace == 0;
            return true;
        }

        let mut is_summable = false;

        'outer: for i in 0..(25 as usize) {
            for j in i..25 {
                if self.values[i] + self.values[j] == *v {
                    is_summable = true;
                    break 'outer;
                }
            }
        }
        self.values[self.index_to_replace] = *v;
        self.index_to_replace = (self.index_to_replace + 1) % 25;

        return is_summable;
    }
}


fn main() {
    let numbers: Vec<usize> = include_str!("../../data/2020_p09_input.txt")
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();

    let mut xmas_win = XMASWindow::new();

    for num in numbers.iter() {
        if !xmas_win.roll_with(num) {
            println!("num that was not formable with #'s from preamble: {}", num);
            break;
        }
    }
}
