use crate::runner::{Opts, Runner};
use anyhow::Result;
use clap::Parser;

use std::iter::Peekable;

mod day_1;
mod day_2;
mod day_3;
mod runner;

pub struct Lines {
    lines: Peekable<Box<dyn Iterator<Item = String>>>,
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next()
    }
}

impl Lines {
    pub fn new(lines: Box<dyn Iterator<Item = String>>) -> Self {
        Self {
            lines: lines.peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<&String> {
        self.lines.peek()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let runner = Runner::new()?;
    let input = runner.get_input(opts.day, opts.force_download).await?;

    let implementation = match (opts.day.day_number(), opts.part.part_number()) {
        (1, 1) => day_1::day_1_1,
        (1, 2) => day_1::day_1_2,
        (2, 1) => day_2::day_2_1,
        (2, 2) => day_2::day_2_2,
        (3, 1) => day_3::day_3_1,
        (3, 2) => day_3::day_3_2,
        _ => panic!(
            "Day {} part {} not implemented",
            opts.day.day_number(),
            opts.part.part_number()
        ),
    };

    let result = implementation(input)?;

    println!("{}", result);
    println!();

    if opts.send {
        println!("Sending the answer");
        runner
            .send_result(opts.day, opts.part, result.to_string())
            .await?;
    }
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    impl Lines {
        pub fn from_text(s: &'static str) -> Self {
            Lines::new(Box::new(s.lines().map(|line| line.to_string())))
        }
    }

    pub fn aoc_test<Output: ToString>(
        input: &'static str,
        implementation: fn(Lines) -> Result<Output>,
        expected: Output,
    ) {
        let lines = Lines::from_text(input);
        assert_eq!(
            implementation(lines).unwrap().to_string(),
            expected.to_string()
        );
    }
}
