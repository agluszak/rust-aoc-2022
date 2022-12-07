use crate::Lines;
use anyhow::{anyhow, Result};

use std::collections::HashSet;

fn find_marker(input: &str, window_size: usize) -> Result<usize> {
    for (i, window) in input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
    {
        if window.iter().collect::<HashSet<_>>().len() == window_size {
            return Ok(i + window_size);
        }
    }
    Err(anyhow!("No solution found"))
}

pub fn day_6_1(mut lines: Lines) -> Result<usize> {
    let input = lines.next().ok_or_else(|| anyhow!("No input"))?;
    find_marker(&input, 4)
}

pub fn day_6_2(mut lines: Lines) -> Result<usize> {
    let input = lines.next().ok_or_else(|| anyhow!("No input"))?;
    find_marker(&input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::aoc_test;

    #[test]
    fn day_6_1_test() {
        aoc_test("mjqjpqmgbljsphdztnvjfqwrcgsmlb", day_6_1, 7);
        aoc_test("bvwbjplbgvbhsrlpgdmjqwftvncz", day_6_1, 5);
        aoc_test("nppdvjthqldpwncqszvftbrmjlhg", day_6_1, 6);
        aoc_test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", day_6_1, 10);
        aoc_test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", day_6_1, 11);
    }

    #[test]
    fn day_6_2_test() {
        aoc_test("mjqjpqmgbljsphdztnvjfqwrcgsmlb", day_6_2, 19);
        aoc_test("bvwbjplbgvbhsrlpgdmjqwftvncz", day_6_2, 23);
        aoc_test("nppdvjthqldpwncqszvftbrmjlhg", day_6_2, 23);
        aoc_test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", day_6_2, 29);
        aoc_test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", day_6_2, 26);
    }
}
