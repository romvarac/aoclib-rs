use aoclib::{Runner, Selector, run_solution};

mod aoc2025_01;
mod aoc2025_02;
mod aoc2025_03;
mod aoc2025_04;
mod aoc2025_05;

use aoc2025_01::*;
use aoc2025_02::*;
use aoc2025_03::*;
use aoc2025_04::*;
use aoc2025_05::*;

fn main() {
    run_2025(Selector::One(5));
}

fn run_2025(which: Selector) {
    let mut day01 = Aoc2025_01::new();
    let mut day02 = Aoc2025_02::new();
    let mut day03 = Aoc2035_03::new();
    let mut day04 = Aoc2045_04::new();
    let mut day05 = Aoc2055_05::new();

    let mut days: Vec<&mut dyn Runner> =
        vec![&mut day01, &mut day02, &mut day03, &mut day04, &mut day05];
    match which {
        Selector::All => {
            for d in days {
                run_solution(d);
            }
        }
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            run_solution(*d);
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            run_solution(*d);
        }
    }
}
