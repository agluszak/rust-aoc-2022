use crate::Lines;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("Invalid shape: {}", s)),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn for_outcome(opponent_shape: Shape, expected: Outcome) -> Self {
        match expected {
            Outcome::Win => match opponent_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Loss => match opponent_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => opponent_shape,
        }
    }
}

struct Round {
    opponent_shape: Shape,
    player_shape: Shape,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = s.split_whitespace();
        let opponent_shape = shapes
            .next()
            .ok_or_else(|| anyhow!("Missing opponent shape"))?
            .parse()?;
        let player_shape = shapes
            .next()
            .ok_or_else(|| anyhow!("Missing player shape"))?
            .parse()?;
        Ok(Round {
            opponent_shape,
            player_shape,
        })
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        match (self.opponent_shape, self.player_shape) {
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    fn score(&self) -> i32 {
        self.outcome().score() + self.player_shape.score()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Invalid outcome: {}", s)),
        }
    }
}

struct RoundWithOutcome {
    opponent_shape: Shape,
    outcome: Outcome,
}

impl FromStr for RoundWithOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = s.split_whitespace();
        let opponent_shape = shapes
            .next()
            .ok_or_else(|| anyhow!("Missing opponent shape"))?
            .parse()?;
        let outcome = shapes
            .next()
            .ok_or_else(|| anyhow!("Missing outcome"))?
            .parse()?;
        Ok(RoundWithOutcome {
            opponent_shape,
            outcome,
        })
    }
}

impl RoundWithOutcome {
    fn score(&self) -> i32 {
        self.outcome.score() + Shape::for_outcome(self.opponent_shape, self.outcome).score()
    }
}

pub fn day_2_1(input: Lines) -> Result<i32> {
    let rounds = input
        .map(|line| line.parse())
        .collect::<Result<Vec<Round>, _>>()?;
    Ok(rounds.iter().map(|round| round.score()).sum())
}

pub fn day_2_2(input: Lines) -> Result<i32> {
    let rounds = input
        .map(|line| line.parse())
        .collect::<Result<Vec<RoundWithOutcome>, _>>()?;
    Ok(rounds.iter().map(|round| round.score()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::aoc_test;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn day_2_1_test() {
        aoc_test(INPUT, day_2_1, 15);
    }

    #[test]
    fn day_2_2_test() {
        aoc_test(INPUT, day_2_2, 12);
    }

    #[test]
    fn day_2_2_test_2() {
        aoc_test(
            "A Z
C X
B Y",
            day_2_2,
            15,
        );
    }
}
