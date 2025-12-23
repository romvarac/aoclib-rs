use std::str::FromStr;

use aoclib::{Runner, output, read_lines};

#[derive(Default)]
pub struct Aoc2055_05 {
    ranges: Vec<Range>,
    ingredients: Vec<usize>,
}

impl Aoc2055_05 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2055_05 {
    fn name(&self) -> (usize, usize) {
        (2025, 5)
    }

    fn parse(&mut self) {
        let name = self.name();
        let lines = read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
        for line in lines {
            if line.contains("-") {
                self.ranges.push(line.parse().unwrap());
                continue;
            }
            self.ingredients.push(line.parse::<usize>().unwrap());
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let counter = self
            .ingredients
            .iter()
            .filter(|ingredient| self.ranges.iter().any(|range| range.contains(**ingredient)))
            .count();
        output(counter)
    }

    fn part2(&mut self) -> Vec<String> {
        self.ranges.sort();
        let mut total = 0;
        let mut range_list = self.ranges.iter();
        let mut curr_range = *range_list.next().unwrap();

        for range in range_list {
            if range.start <= curr_range.end {
                curr_range.extend_by(range);
                continue;
            }

            total += curr_range.dist();
            curr_range = *range;
        }

        total += curr_range.dist();
        output(total)
    }
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, ingredient: usize) -> bool {
        ingredient >= self.start && ingredient <= self.end
    }

    fn dist(&self) -> usize {
        self.end - self.start + 1
    }

    fn extend_by(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
        self.end = self.end.max(other.end);
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
