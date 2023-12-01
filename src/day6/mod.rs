use crate::solver::Solver;
use std::collections::HashSet;

const START_OF_PACKET_LENGTH: usize = 4;
const START_OF_MESSAGE_LENGTH: usize = 14;

pub struct Day6Solver {}
impl Day6Solver {
    // Rationale: Move through candidate messages left to right.
    // Collect the candidate message in a hashset.
    // If the resulting hashset is the same size as the length
    // of an expected message, then all characters are unique.
    fn solve_part2_with_hashset(chars: &[u8]) -> usize {
        for i in START_OF_MESSAGE_LENGTH..chars.len() {
            let start_of_message: HashSet<_> =
                chars[i - START_OF_MESSAGE_LENGTH..i].iter().collect();
            if start_of_message.len() == START_OF_MESSAGE_LENGTH {
                return i;
            }
        }

        unreachable!()
    }

    // Rationale: Move through candidate messages left to right.
    // Check all pairs of characters within a message from left to right.
    // If any are equal, move to the next candidate.
    fn solve_part2_with_naive_loop(chars: &[u8]) -> usize {
        'outer: for marker in START_OF_MESSAGE_LENGTH..chars.len() {
            let start_of_message = &chars[marker - START_OF_MESSAGE_LENGTH..marker];

            for i in 0..(START_OF_MESSAGE_LENGTH - 1) {
                for j in (i + 1)..START_OF_MESSAGE_LENGTH {
                    if start_of_message[i] == start_of_message[j] {
                        continue 'outer;
                    }
                }
            }

            return marker;
        }

        unreachable!()
    }

    // Rationale: Move through candidate messages left to right.
    // Check all pairs of characters within a message from left to right.
    // If any are equal, we apply a trick to move to the next candidate.
    // Whereas the previous implementation just moved to the next one to the right,
    // we can realize that if the Xth character in the message matched with the
    // Yth character such that X < Y, then there's no point in moving to a next
    // candidate message that still contains the two identical characters.
    // Therefore, we can proceed with the candidate starting at character X+1,
    // which potentially skips many characters everytime.
    fn solve_part2_with_smart_loop(chars: &[u8]) -> usize {
        let mut marker = START_OF_MESSAGE_LENGTH;
        'outer: loop {
            let start_of_message = &chars[marker - START_OF_MESSAGE_LENGTH..marker];

            for i in 0..(START_OF_MESSAGE_LENGTH - 1) {
                for j in (i + 1)..START_OF_MESSAGE_LENGTH {
                    if start_of_message[i] == start_of_message[j] {
                        marker += i + 1;
                        continue 'outer;
                    }
                }
            }

            return marker;
        }
    }

    // Rationale: this is the same as the previous one, except that we compare
    // characters within a message starting from the end of the message, and
    // making our way back to the start of the message. That way, the trick
    // presented in the previous solution will be maximized, as in we will skip
    // the maximum number of characters for each candidate.
    fn solve_part2_with_even_smarter_loop(chars: &[u8]) -> usize {
        let mut marker = START_OF_MESSAGE_LENGTH;
        'outer: loop {
            let start_of_message = &chars[marker - START_OF_MESSAGE_LENGTH..marker];

            for i in (0..(START_OF_MESSAGE_LENGTH - 1)).rev() {
                for j in (i + 1)..START_OF_MESSAGE_LENGTH {
                    if start_of_message[i] == start_of_message[j] {
                        marker += i + 1;
                        continue 'outer;
                    }
                }
            }

            return marker;
        }
    }
}

impl Solver for Day6Solver {
    fn solve_part1() {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        for i in START_OF_PACKET_LENGTH..chars.len() {
            let start_of_packet = &chars[i - START_OF_PACKET_LENGTH..i];

            if start_of_packet[0] != start_of_packet[1]
                && start_of_packet[0] != start_of_packet[2]
                && start_of_packet[0] != start_of_packet[3]
                && start_of_packet[1] != start_of_packet[2]
                && start_of_packet[1] != start_of_packet[3]
                && start_of_packet[2] != start_of_packet[3]
            {
                println!("Characters to process to find the start-of-packet marker: {i}");
                return;
            }
        }
    }

    fn solve_part2() {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        println!(
            "Characters to process to find the start-of-message marker: {}",
            Self::solve_part2_with_even_smarter_loop(&chars)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part2_hashset(b: &mut Bencher) {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        b.iter(|| Day6Solver::solve_part2_with_hashset(&chars));
    }

    #[bench]
    fn bench_part2_naive_loop(b: &mut Bencher) {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        b.iter(|| Day6Solver::solve_part2_with_naive_loop(&chars));
    }

    #[bench]
    fn bench_part2_smart_loop(b: &mut Bencher) {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        b.iter(|| Day6Solver::solve_part2_with_smart_loop(&chars));
    }

    #[bench]
    fn bench_part2_even_smarter_loop(b: &mut Bencher) {
        let chars = std::fs::read("src/day6/input.txt").unwrap();

        b.iter(|| Day6Solver::solve_part2_with_even_smarter_loop(&chars));
    }
}
