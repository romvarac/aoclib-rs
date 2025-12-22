use std::str::FromStr;

use aoclib::{ParseLine, Runner, output, read_lines};

pub struct Aoc2035_03 {
    power_banks: Vec<PowerBank>,
}

impl Aoc2035_03 {
    pub fn new() -> Self {
        Self {
            power_banks: Vec::default(),
        }
    }
}

impl Runner for Aoc2035_03 {
    fn name(&self) -> (usize, usize) {
        (2025, 3)
    }

    fn parse(&mut self) {
        let name = self.name();
        let lines = read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
        self.power_banks = lines.parse_lines()
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.power_banks
                .iter()
                .map(|bank| bank.best_number_by_digit(2))
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.power_banks
                .iter()
                .map(|bank| bank.best_number_by_digit(12))
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
struct PowerBank {
    bank: Vec<u8>,
}

impl PowerBank {
    pub fn best_number_by_digit(&self, digits: usize) -> usize {
        let mut start = 0;
        let mut answer = 0;
        let len_bank = self.bank.len();
        for digit in 0..digits {
            let limit = digits - digit - 1;
            let Some((pos, &next_digit)) = self.bank[start..len_bank - limit]
                .iter()
                .enumerate()
                .rev()
                .max_by(|a, b| a.1.cmp(b.1))
            else {
                panic!("")
            };
            start += pos + 1;
            answer = (answer * 10) + next_digit as usize;
        }

        answer
    }
}

impl FromStr for PowerBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerBank {
            bank: s.chars().map(|c| c as u8 - b'0').collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let power_bank = PowerBank {
            bank: vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        };

        println!("{}", power_bank.bank[0]);
        println!("{}", power_bank.best_number_by_digit(2));
    }
}
