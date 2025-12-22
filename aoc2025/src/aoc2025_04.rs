use std::collections::HashSet;

use aoclib::{Runner, output, read_lines};

#[derive(Default)]
pub struct Aoc2045_04 {
    rows: isize,
    colums: isize,
    rolls: HashSet<(isize, isize)>,
}

impl Aoc2045_04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2045_04 {
    fn name(&self) -> (usize, usize) {
        (2025, 4)
    }

    fn parse(&mut self) {
        let name = self.name();
        let lines = read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
        self.rows = lines.len() as isize;
        self.colums = lines[0].len() as isize;

        for (row, line) in lines.iter().enumerate() {
            let row = row as isize;
            for (col, ch) in line.chars().enumerate() {
                if ch != '@' {
                    continue;
                }

                self.rolls.insert((row, col as isize));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.find_accessible().len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total_remove = 0;
        loop {
            let removed = self.find_accessible();
            if removed.is_empty() {
                break;
            }

            total_remove += removed.len();
            for roll in removed {
                self.rolls.remove(&roll);
            }
        }

        output(total_remove)
    }
}

impl Aoc2045_04 {
    const DIRS: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];

    fn find_accessible(&self) -> Vec<(isize, isize)> {
        self.rolls
            .iter()
            .filter(|roll| {
                Self::DIRS
                    .iter()
                    .map(|point| (roll.0 + point.0, roll.1 + point.1))
                    .filter(|point| self.rolls.contains(point))
                    .count()
                    < 4
            })
            .copied()
            .collect()
    }
}
