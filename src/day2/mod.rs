use std::fs;

use crate::solver::Solver;

#[derive(Copy, Clone)]
enum Hand {
  Rock,
  Paper,
  Scissors,
}

#[derive(Copy, Clone)]
enum FightResult {
  Win,
  Draw,
  Loss,
}

impl FightResult {
  fn points(&self) -> i32 {
    match self {
      Self::Win => 6,
      Self::Draw => 3,
      Self::Loss => 0,
    }
  }
}

impl Hand {
  fn fight(&self, other_hand: Hand) -> FightResult {
    match (self, other_hand) {
      (Self::Rock, Self::Rock) | (Self::Paper, Self::Paper) | (Self::Scissors, Self::Scissors) => FightResult::Draw,
      (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) => FightResult::Win,
      _ => FightResult::Loss,
    }
  }

  fn points(&self) -> i32 {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }

  fn hand_to_play_if(&self, expected_result: FightResult) -> Self {
    match (self, expected_result) {
     (Self::Rock, FightResult::Draw) | (Self::Paper, FightResult::Loss) | (Self::Scissors, FightResult::Win) => Self::Rock,
     (Self::Rock, FightResult::Win) | (Self::Paper, FightResult::Draw) | (Self::Scissors, FightResult::Loss) => Self::Paper,
     _ => Self::Scissors,
    }
  }
}

pub struct Day2Solver {}
impl Solver for Day2Solver {
  fn solve_part1() {
    let file = fs::read_to_string("src/day2/input.txt").unwrap();
    let mut total_points = 0;

    for line in file.lines() {
      let (adversary_symbol, our_symbol) = line.split_once(' ').unwrap();
      let adversary_hand = match adversary_symbol {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => unreachable!(),
      };
      let our_hand = match our_symbol {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => unreachable!(),
      };

      let fight_result = our_hand.fight(adversary_hand);
      total_points += fight_result.points() + our_hand.points();
    }

    println!("Total points: {total_points}");
  }

  fn solve_part2() {
    let file = fs::read_to_string("src/day2/input.txt").unwrap();
    let mut total_points = 0;

    for line in file.lines() {
      let (adversary_symbol, fight_symbol) = line.split_once(' ').unwrap();
      let adversary_hand = match adversary_symbol {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => unreachable!(),
      };
      let fight_result = match fight_symbol {
        "X" => FightResult::Loss,
        "Y" => FightResult::Draw,
        "Z" => FightResult::Win,
        _ => unreachable!(),
      };

      let our_hand = adversary_hand.hand_to_play_if(fight_result);
      total_points += fight_result.points() + our_hand.points();
    }

    println!("Total points: {total_points}");
  }
}