use std::str::FromStr;

use aoclib::{Runner, output, read_lines};

pub struct Aoc2025_02 {
    ranges: Vec<Range>,
}

impl Aoc2025_02 {
    pub fn new() -> Self {
        Self {
            ranges: Vec::default(),
        }
    }
}

impl Runner for Aoc2025_02 {
    fn name(&self) -> (usize, usize) {
        (2025, 2)
    }

    fn parse(&mut self) {
        let name = self.name();
        let lines = read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
        self.ranges = lines[0]
            .split(",")
            .map(|range| range.parse().unwrap())
            .collect()
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.ranges
                .iter()
                .map(Range::sum_invalid_ids)
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(
            self.ranges
                .iter()
                .map(Range::sum_multi_invalid_ids)
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn sum_invalid_ids(&self) -> usize {
        (self.start..=self.end)
            .filter(|num| {
                let half_digits = num.ilog10().div_ceil(2);
                let mod_val = 10usize.pow(half_digits);
                let lower_half = num % mod_val;
                let upper_half = num / mod_val;
                lower_half == upper_half
            })
            .sum()
    }

    fn sum_multi_invalid_ids(&self) -> usize {
        let mut sum_invalid = 0;

        for num in self.start..=self.end {
            let half_digits = num.ilog10().div_ceil(2);

            for digit in 1..=half_digits {
                let mod_val = 10usize.pow(digit);
                let selected = num % mod_val;
                let mut rested = num / mod_val;

                if selected == 0 || selected.ilog(10) + 1 != digit {
                    continue;
                }

                let mut found = true;
                while rested > 0 {
                    found = rested % mod_val == selected;
                    if !found {
                        break;
                    }

                    rested /= mod_val;
                }

                if found {
                    sum_invalid += num;
                    break;
                }
            }
        }

        sum_invalid
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();
        Ok(Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multi_invalid_id() {
        let range = Range {
            start: 2121212118,
            end: 2121212124,
        };

        println!("{}", range.sum_multi_invalid_ids());
    }
}
