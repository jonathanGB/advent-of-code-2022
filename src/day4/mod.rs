use crate::solver::Solver;
use unbounded_interval_tree::interval_tree::IntervalTree;

fn parse_range_str(range: &str) -> (i32, i32) {
    match range.split_once('-') {
        Some((start, end)) => (start.parse().unwrap(), end.parse().unwrap()),
        None => unreachable!(),
    }
}

pub struct Day4Solver {}
impl Solver for Day4Solver {
    fn solve_part1() {
        let file = std::fs::read_to_string("src/day4/input.txt").unwrap();
        let mut num_of_full_overlaps = 0;

        for line in file.lines() {
            let (fist_range_str, second_range_str) = line.split_once(',').unwrap();
            let (first_start, first_end) = parse_range_str(fist_range_str);
            let (second_start, second_end) = parse_range_str(second_range_str);
            let first_interval = first_start..=first_end;
            let second_interval = second_start..=second_end;

            if IntervalTree::from([first_interval.clone()]).contains_interval(&second_interval)
                || IntervalTree::from([second_interval.clone()]).contains_interval(&first_interval)
            {
                num_of_full_overlaps += 1;
            }
        }
        println!("Total of full overlaps is {num_of_full_overlaps}.");
    }

    fn solve_part2() {
        let file = std::fs::read_to_string("src/day4/input.txt").unwrap();
        let mut num_of_full_overlaps = 0;

        for line in file.lines() {
            let (fist_range_str, second_range_str) = line.split_once(',').unwrap();
            let (first_start, first_end) = parse_range_str(fist_range_str);
            let (second_start, second_end) = parse_range_str(second_range_str);
            let first_interval = first_start..=first_end;
            let second_interval = second_start..=second_end;

            if !IntervalTree::from([first_interval])
                .get_interval_overlaps(&second_interval)
                .is_empty()
            {
                num_of_full_overlaps += 1;
            }
        }
        println!("Total of overlaps is {num_of_full_overlaps}.");
    }
}
