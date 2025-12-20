use std::str::FromStr;

use aoclib::{ParseLine, Runner, output};

pub struct Aoc2025_01 {
    start_point: isize,
    turns: Vec<Turn>,
}

impl Aoc2025_01 {
    pub fn new() -> Self {
        Self {
            turns: Vec::new(),
            start_point: 50,
        }
    }
}

impl Runner for Aoc2025_01 {
    fn name(&self) -> (usize, usize) {
        (2025, 1)
    }

    fn parse(&mut self) {
        let name = self.name();
        let lines = aoclib::read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
        self.turns = lines.parse_lines();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut counter = 0;
        let mut setting: isize = self.start_point;
        for turn in &self.turns {
            match turn {
                Turn::Left(num) => setting = (setting - num).rem_euclid(100),
                Turn::Right(num) => setting = (setting + num) % 100,
            }

            if setting == 0 {
                counter += 1;
            }
        }

        output(format!("{}", counter))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut counter = 0;
        let mut setting = self.start_point;

        for turn in &self.turns {
            let (dir, &num) = match turn {
                Turn::Left(num) => (-1, num),
                Turn::Right(num) => (1, num),
            };

            let mut num = num;
            while num > 0 {
                setting = (setting + dir).rem_euclid(100);
                if setting == 0 {
                    counter += 1;
                }
                num -= 1;
            }
        }

        output(format!("{}", counter))
    }
}

#[derive(Debug)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_left = &s[0..1] == "L";
        let amount = s[1..].parse::<isize>().unwrap();

        if is_left {
            return Ok(Turn::Left(amount));
        }

        Ok(Turn::Right(amount))
    }
}
