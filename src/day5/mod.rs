use regex::{Captures, Regex};
use std::iter::Peekable;
use std::str::Lines;

use crate::solver::Solver;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

lazy_static! {
    static ref INSTRUCTIONS_RE: Regex = Regex::new(r"^move (\d+) from (\d) to (\d)$").unwrap();
}

// 3 chars per stack, and 1 whitespace in between stacks.
const STACK_WIDTH: usize = 4;

pub struct Day5Solver<'a> {
    lines: Peekable<Lines<'a>>,
    stacks: Stacks,
}

struct Instruction {
    num_moves: usize,
    stack_index_from: usize,
    stack_index_to: usize,
}

impl<'a> Day5Solver<'a> {
    fn new(mut lines: Peekable<Lines<'a>>) -> Self {
        // Add one for the last stack, which has no buffer.
        let num_of_stacks = (lines.peek().unwrap().len() + 1) / STACK_WIDTH;
        Self {
            lines,
            stacks: vec![Stack::default(); num_of_stacks],
        }
    }

    fn parse_initial_configuration(&mut self) {
        loop {
            let line = match self.lines.peek() {
                Some(line) => line,
                None => unreachable!(),
            };

            if line.starts_with(" 1") {
                // Skip the line with the indices.
                self.lines.next();
                // Skip the empty line afterwards.
                self.lines.next();
                break;
            }

            for (i, stack) in self.stacks.iter_mut().enumerate() {
                match &line[i * STACK_WIDTH..] {
                    window if window.starts_with("  ") => continue,
                    window if window.starts_with('[') => {
                        // The 2nd character in the window is the crate identifier.
                        stack.push(window.chars().nth(1).unwrap())
                    }
                    _ => unreachable!(),
                }
            }

            // We just peeked at the current line. Consume it now.
            self.lines.next().unwrap();
        }

        // We built the stacks upside down, so we reverse them.
        self.stacks.iter_mut().for_each(|stack| stack.reverse());
    }

    fn parse_instruction_detail(instruction: &Captures, index: usize) -> usize {
        instruction.get(index).unwrap().as_str().parse().unwrap()
    }

    fn parse_intruction(instruction: &str) -> Instruction {
        let instruction = INSTRUCTIONS_RE.captures(instruction).unwrap();
        let num_moves = Self::parse_instruction_detail(&instruction, 1);
        // Decrement the parsed index, as the instruction is 1th-based
        // whereas the stacks are 0th-based.
        let stack_index_from = Self::parse_instruction_detail(&instruction, 2) - 1;
        let stack_index_to = Self::parse_instruction_detail(&instruction, 3) - 1;

        Instruction {
            num_moves,
            stack_index_from,
            stack_index_to,
        }
    }

    fn go_through_crate_mover_9000_procedure(&mut self) {
        for line in &mut self.lines {
            let Instruction {
                num_moves,
                stack_index_from,
                stack_index_to,
            } = Self::parse_intruction(line);
            for _ in 0..num_moves {
                let crate_moved = self.stacks[stack_index_from].pop().unwrap();
                self.stacks[stack_index_to].push(crate_moved);
            }
        }
    }

    fn go_through_crate_mover_9001_procedure(&mut self) {
        for line in &mut self.lines {
            let Instruction {
                num_moves,
                stack_index_from,
                stack_index_to,
            } = Self::parse_intruction(line);

            let stack_from_len = self.stacks[stack_index_from].len();
            let crates_moved: Vec<_> = self.stacks[stack_index_from]
                .drain(stack_from_len - num_moves..)
                .collect();
            self.stacks[stack_index_to].extend(crates_moved);
        }
    }

    fn print_top_crates(&self) {
        println!(
            "{}",
            self.stacks
                .iter()
                .map(|stack| stack.last().unwrap())
                .collect::<String>()
        );
    }
}

impl Solver for Day5Solver<'_> {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day5/input.txt").unwrap();

        let mut solver = Day5Solver::new(file.lines().peekable());
        solver.parse_initial_configuration();
        solver.go_through_crate_mover_9000_procedure();
        solver.print_top_crates();
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day5/input.txt").unwrap();

        let mut solver = Day5Solver::new(file.lines().peekable());
        solver.parse_initial_configuration();
        solver.go_through_crate_mover_9001_procedure();
        solver.print_top_crates();
    }
}
