use std::collections::HashSet;

use crate::solver::Solver;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: Direction,
    length: u8,
}

impl From<&str> for Motion {
    fn from(value: &str) -> Self {
        let (direction, length) = value.split_once(' ').unwrap();
        Self {
            direction: direction.into(),
            length: length.parse().unwrap(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Position {
    // x increases as we go right, and decreases as we go left.
    x: i16,
    // y increases as we go up, and decreases as we go down.
    y: i16,
}

struct Rope {
    knots: Vec<Position>,
    tail_index: usize,
    visited_tail_positions: HashSet<Position>,
}

impl Rope {
    fn try_new(knots: usize) -> Result<Self, String> {
        if knots < 2 {
            Err(format!("`knots` must be at least 2, got {knots}"))
        } else {
            Ok(Self {
                knots: vec![Position::default(); knots],
                tail_index: knots - 1,
                visited_tail_positions: HashSet::from([Position::default()]),
            })
        }
    }

    fn mut_head(&mut self) -> &mut Position {
        &mut self.knots[0]
    }

    fn tail(&self) -> Position {
        self.knots[self.tail_index]
    }

    fn apply_motion(&mut self, motion: Motion) {
        'motion: for _ in 0..motion.length {
            match motion.direction {
                Direction::Up => {
                    self.mut_head().y += 1;
                }
                Direction::Right => {
                    self.mut_head().x += 1;
                }
                Direction::Down => {
                    self.mut_head().y -= 1;
                }
                Direction::Left => {
                    self.mut_head().x -= 1;
                }
            }

            for knot in 0..self.tail_index {
                let new_tail_position =
                    Self::new_tail_position(self.knots[knot], self.knots[knot + 1]);

                // New tail has not moved, it is pointless to pursue this motion further.
                if new_tail_position == self.knots[knot + 1] {
                    continue 'motion;
                }

                self.knots[knot + 1] = new_tail_position;
            }

            self.visited_tail_positions.insert(self.tail());
        }
    }

    fn new_tail_position(relative_head: Position, mut relative_tail: Position) -> Position {
        let tail_to_head_x_distance = relative_head.x - relative_tail.x;
        let tail_to_head_y_distance = relative_head.y - relative_tail.y;

        let mut tail_too_far_horizontally = false;
        let mut tail_too_far_vertically = false;

        // Update the x position of the tail if it's too far.
        if tail_to_head_x_distance > 1 {
            relative_tail.x += 1;
            tail_too_far_horizontally = true;
        } else if tail_to_head_x_distance < -1 {
            relative_tail.x -= 1;
            tail_too_far_horizontally = true;
        }

        // Update the y position of the tail if it's too far.
        if tail_to_head_y_distance > 1 {
            relative_tail.y += 1;
            tail_too_far_vertically = true;
        } else if tail_to_head_y_distance < -1 {
            relative_tail.y -= 1;
            tail_too_far_vertically = true;
        }

        if tail_too_far_horizontally && tail_too_far_vertically {
            // Nothing to do, it's already fully moved.
        } else if tail_too_far_horizontally {
            relative_tail.y += tail_to_head_y_distance;
        } else if tail_too_far_vertically {
            relative_tail.x += tail_to_head_x_distance;
        }

        relative_tail
    }
}

pub struct Day9Solver {}

impl Day9Solver {
    fn read_file() -> String {
        std::fs::read_to_string("src/day9/input.txt").unwrap()
    }
}

impl Solver for Day9Solver {
    fn solve_part1() {
        let mut rope = Rope::try_new(2).unwrap();
        Self::read_file()
            .lines()
            .map(Motion::from)
            .for_each(|motion| rope.apply_motion(motion));

        println!(
            "The tail has visited {} positions.",
            rope.visited_tail_positions.len()
        );
    }

    fn solve_part2() {
        let mut rope = Rope::try_new(10).unwrap();
        Self::read_file()
            .lines()
            .map(Motion::from)
            .for_each(|motion| rope.apply_motion(motion));

        println!(
            "The tail has visited {} positions.",
            rope.visited_tail_positions.len()
        );
    }
}
