use aoclib::{Runner, output, read_lines};

#[derive(Default)]
pub struct Aoc2025_06 {
    lines: Vec<String>,
}

impl Aoc2025_06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Aoc2025_06 {
    fn name(&self) -> (usize, usize) {
        (2025, 6)
    }

    fn parse(&mut self) {
        let name = self.name();
        self.lines = read_lines(format!("inputs/{}_{}.txt", name.0, name.1));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut worksheets: Vec<Worksheet> = Vec::default();
        for line in &self.lines {
            for (index, input) in line.split_whitespace().enumerate() {
                if Worksheet::is_operation(&input) {
                    worksheets[index].set_operation(input);
                    continue;
                }

                let problem = Worksheet::parse_problem(input);
                if worksheets.len() <= index {
                    worksheets.push(Worksheet {
                        problems: vec![problem],
                        operation: Operation::Unknown,
                    });
                    continue;
                }

                worksheets[index].problems.push(problem);
            }
        }
        output(
            worksheets
                .iter()
                .map(|worksheet| worksheet.calculate())
                .sum::<usize>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut worksheets: Vec<Worksheet> = Vec::default();
        let mut rotated = vec![String::new(); self.lines[0].len()];
        for line in self.lines.iter() {
            for (col, ch) in line.chars().enumerate() {
                rotated[col].push(ch);
            }
        }

        let mut index = 0;
        for problem in rotated {
            let input = problem.trim();
            if input.is_empty() {
                index += 1;
                continue;
            }

            if input.contains("*") {
                let problem = input[0..input.len() - 1].trim().parse::<usize>().unwrap();
                worksheets.push(Worksheet {
                    problems: vec![problem],
                    operation: Operation::Multiplied,
                });
                continue;
            }

            if input.contains("+") {
                let problem = input[0..input.len() - 1].trim().parse::<usize>().unwrap();
                worksheets.push(Worksheet {
                    problems: vec![problem],
                    operation: Operation::Added,
                });
                continue;
            }

            worksheets[index]
                .problems
                .push(Worksheet::parse_problem(input.trim()));
        }

        output(
            worksheets
                .iter()
                .map(|worksheet| worksheet.calculate())
                .sum::<usize>(),
        )
    }
}

#[derive(Debug)]
enum Operation {
    Unknown,
    Added,
    Multiplied,
}

#[derive(Debug)]
struct Worksheet {
    problems: Vec<usize>,
    operation: Operation,
}

impl Worksheet {
    fn is_operation(input: &str) -> bool {
        input.contains("+") || input.contains("*")
    }

    fn set_operation(&mut self, operation: &str) {
        self.operation = match operation {
            "+" => Operation::Added,
            "*" => Operation::Multiplied,
            _ => Operation::Unknown,
        };
    }

    fn parse_problem(problem: &str) -> usize {
        problem.parse::<usize>().unwrap()
    }

    fn calculate(&self) -> usize {
        match self.operation {
            Operation::Added => self.problems.iter().sum(),
            Operation::Multiplied => {
                let mut mul = 1;
                for problem in self.problems.iter() {
                    mul *= problem;
                }
                mul
            }
            Operation::Unknown => 0,
        }
    }
}
