use crate::solver::Solver;

const CRT_SCREEN_WIDTH: usize = 40;
const CRT_SCREEN_HEIGHT: usize = 6;

pub struct Day10Solver {}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if value.starts_with("noop") {
            Self::Noop
        } else if value.starts_with("addx") {
            let (_, v) = value.split_once(' ').unwrap();
            Self::Addx(v.parse().unwrap())
        } else {
            unreachable!()
        }
    }
}

struct Program {
    // Starts at 1, and increments every cycle.
    cycle: i32,
    // Current value in the `X` register.
    register: i32,
    // Only used for part 1.
    sum_signal_strengths: i32,
    // Only used for part 2.
    crt_screen: CrtScreen,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            cycle: 1,
            register: 1,
            sum_signal_strengths: 0,
            crt_screen: CrtScreen::default(),
        }
    }
}

#[derive(Clone, Copy)]
enum Pixel {
    Lit,
    Dark,
}

impl From<&Pixel> for char {
    fn from(value: &Pixel) -> Self {
        match *value {
            Pixel::Lit => '#',
            Pixel::Dark => '.',
        }
    }
}

struct CrtScreen {
    screen: [[Pixel; CRT_SCREEN_WIDTH]; CRT_SCREEN_HEIGHT],
}

impl Default for CrtScreen {
    fn default() -> Self {
        Self {
            screen: [[Pixel::Dark; CRT_SCREEN_WIDTH]; CRT_SCREEN_HEIGHT],
        }
    }
}

impl std::fmt::Display for CrtScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.screen {
            let row: String = row.iter().map(char::from).collect();
            write!(f, "{}\n", row).unwrap();
        }

        Ok(())
    }
}

impl Program {
    fn compute_sum_signal_strengths(instructions: impl Iterator<Item = Instruction>) -> i32 {
        let mut program = Self::default();

        for instruction in instructions {
            program.run_compute_sum_signal_strengths_cycle(None);

            if let Instruction::Addx(v) = instruction {
                program.run_compute_sum_signal_strengths_cycle(Some(v));
            }
        }

        program.sum_signal_strengths
    }

    fn run_compute_sum_signal_strengths_cycle(&mut self, update_register: Option<i32>) {
        if self.is_monitored_cycle() {
            self.sum_signal_strengths += self.cycle * self.register;
        }

        self.cycle += 1;
        if let Some(v) = update_register {
            self.register += v;
        }
    }

    fn is_monitored_cycle(&self) -> bool {
        match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => true,
            _ => false,
        }
    }

    fn run_crt(instructions: impl Iterator<Item = Instruction>) -> CrtScreen {
        let mut program = Self::default();

        for instruction in instructions {
            program.run_crt_cycle(None);

            if let Instruction::Addx(v) = instruction {
                program.run_crt_cycle(Some(v));
            }
        }

        program.crt_screen
    }

    fn run_crt_cycle(&mut self, update_register: Option<i32>) {
        let row = (self.cycle - 1) as usize / CRT_SCREEN_WIDTH;
        let col = (self.cycle - 1) % CRT_SCREEN_WIDTH as i32;

        if col == self.register - 1 || col == self.register || col == self.register + 1 {
            self.crt_screen.screen[row][col as usize] = Pixel::Lit;
        }

        self.cycle += 1;
        if let Some(v) = update_register {
            self.register += v;
        }
    }
}

impl Solver for Day10Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day10/input.txt").unwrap();
        let instructions = file.lines().map(Instruction::from);

        println!(
            "The sum of signal strengths is {}.",
            Program::compute_sum_signal_strengths(instructions)
        );
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day10/input.txt").unwrap();
        let instructions = file.lines().map(Instruction::from);

        println!(
            "The CRT screen looks like:\n\n{}",
            Program::run_crt(instructions)
        );
    }
}
