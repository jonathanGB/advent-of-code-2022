use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

fn build_priority_map() -> HashMap<char, u32> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(priority, item)| (item, (priority + 1) as u32))
        .collect()
}

pub struct Day3Solver {}
impl Solver for Day3Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day3/input.txt").unwrap();
        let priority_map = build_priority_map();
        let mut total_priority: u32 = 0;

        for line in file.lines() {
            let line_len = line.len();
            let first_compartment: HashSet<char> = line[..line_len / 2].chars().collect();
            let second_compartment = line[line_len / 2..].chars().collect::<HashSet<char>>();
            let mistaken_item: &char = first_compartment
                .intersection(&second_compartment)
                .next()
                .unwrap();
            total_priority += priority_map[mistaken_item];
        }

        println!("Total priority is {total_priority}");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day3/input.txt").unwrap();
        let priority_map = build_priority_map();
        let mut total_priority: u32 = 0;
        let mut lines = file.lines().peekable();

        while lines.peek().is_some() {
            let first_rupsack: HashSet<char> = lines.next().unwrap().chars().collect();
            let second_rupsack: HashSet<char> = lines.next().unwrap().chars().collect();
            let third_rupsack: HashSet<char> = lines.next().unwrap().chars().collect();
            let first_and_second_rupsack_common: HashSet<char> = first_rupsack
                .intersection(&second_rupsack)
                .map(|item| *item)
                .collect();
            let badge = first_and_second_rupsack_common
                .intersection(&third_rupsack)
                .next()
                .unwrap();

            total_priority += priority_map[badge];
        }

        println!("Total priority is {total_priority}");
    }
}
