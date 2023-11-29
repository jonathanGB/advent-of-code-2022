use std::fs;

use crate::solver::Solver;

pub struct Day1Solver {}

impl Solver for Day1Solver {
    fn solve_part1() {
        let file = fs::read_to_string("src/day1/input.txt").unwrap();
        let mut max_energy = 0;
        let mut curr_energy = 0;
        for line in file.lines() {
            if line.is_empty() {
                max_energy = max_energy.max(curr_energy);
                curr_energy = 0;
                continue;
            }

            curr_energy += line.parse::<i32>().unwrap();
        }

        println!("{max_energy}");
    }

    fn solve_part2() {
        let file = fs::read_to_string("src/day1/input.txt").unwrap();
        let mut max1_energy = 0;
        let mut max2_energy = 0;
        let mut max3_energy = 0;
        let mut curr_energy = 0;
        for line in file.lines() {
            if line.is_empty() {
                if curr_energy > max1_energy {
                    max3_energy = max2_energy;
                    max2_energy = max1_energy;
                    max1_energy = curr_energy
                } else if curr_energy > max2_energy {
                    max3_energy = max2_energy;
                    max2_energy = curr_energy;
                } else if curr_energy > max3_energy {
                    max3_energy = curr_energy;
                }

                curr_energy = 0;
                continue;
            }

            curr_energy += line.parse::<i32>().unwrap();
        }

        let max_total_energy = max1_energy + max2_energy + max3_energy;
        println!(
            "1: {max1_energy}. 2: {max2_energy}. 3: {max3_energy}. Total: {max_total_energy}."
        );
    }
}
