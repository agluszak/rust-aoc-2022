use crate::Lines;
use anyhow::Result;
use anyhow::{anyhow, bail};
use once_cell::sync::Lazy;
use regex::Regex;

struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
    fn execute_order_9000(&mut self, order: &Order) -> Result<()> {
        let from = order.from;
        let to = order.to;
        let n = order.how_many;

        for _ in 0..n {
            let crate_ = self.0[from]
                .pop()
                .ok_or_else(|| anyhow!("No crate to move"))?;
            self.0[to].push(crate_);
        }

        Ok(())
    }

    fn execute_order_9001(&mut self, order: &Order) -> Result<()> {
        let from = order.from;
        let to = order.to;
        let n = order.how_many;

        let moved = {
            let from = self
                .0
                .get_mut(from)
                .ok_or_else(|| anyhow!("No crate to move"))?;
            if from.len() < n {
                bail!("Not enough crates to move");
            }
            from.split_off(from.len() - n)
        };

        let to = self
            .0
            .get_mut(to)
            .ok_or_else(|| anyhow!("No crate to move"))?;
        to.extend(moved);

        Ok(())
    }

    fn read_top(&self) -> String {
        let mut s = String::new();
        for stack in &self.0 {
            if let Some(crate_) = stack.last() {
                s.push(crate_.0);
            } else {
                s.push(' ');
            }
        }
        s
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Crate(char);

struct Parser<'a> {
    index: usize,
    line: &'a str,
}

impl<'a> Parser<'a> {
    fn new(line: &'a str) -> Self {
        Self { index: 0, line }
    }

    fn take<const N: usize>(&mut self) -> Result<[char; N]> {
        let start = self.index;
        let end = start + N;
        let len = self.line.len();
        if end > len {
            bail!("Not enough characters left");
        }
        self.index = end;
        Ok(self.line[start..end]
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap())
    }

    fn parse_crate(&mut self) -> Result<Option<Crate>> {
        let [lbracket, crate_char, rbracket] = self.take::<3>()?;
        match [lbracket, crate_char, rbracket] {
            ['[', crate_char, ']'] => Ok(Some(Crate(crate_char))),
            [' ', ' ', ' '] => Ok(None),
            other => bail!("Invalid crate: {:?}", other),
        }
    }

    fn chars_left(&self) -> usize {
        self.line.len() - self.index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Order {
    how_many: usize,
    from: usize,
    to: usize,
}

static ORDER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

fn parse_order(line: &str) -> Result<Order> {
    let captures = ORDER_REGEX
        .captures(line)
        .ok_or_else(|| anyhow!("Invalid order: {}", line))?;
    let how_many = captures[1].parse()?;
    let from = captures[2].parse::<usize>()? - 1;
    let to = captures[3].parse::<usize>()? - 1;
    Ok(Order { how_many, from, to })
}

fn parse_line(input: String) -> Result<Vec<Option<Crate>>> {
    let mut parser = Parser::new(&input);
    let mut crates = Vec::new();
    while parser.chars_left() > 0 {
        crates.push(parser.parse_crate()?);
        if parser.chars_left() > 0 {
            parser.take::<1>()?;
        }
    }
    Ok(crates)
}

// Input:
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// Output:
// vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']]
fn parse(mut lines: Lines) -> Result<(Stacks, Vec<Order>)> {
    let mut stacks = Vec::new();
    for line in lines.by_ref() {
        if &line[0..3] == " 1 " {
            break;
        }
        let crates = parse_line(line)?;
        for (j, crate_) in crates.into_iter().enumerate() {
            if stacks.len() <= j {
                stacks.push(Vec::new());
            }
            if let Some(crate_) = crate_ {
                stacks[j].push(crate_);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    let stacks = Stacks(stacks);

    lines.next();

    let mut orders = Vec::new();
    for line in lines {
        orders.push(parse_order(&line)?);
    }

    Ok((stacks, orders))
}

pub fn day_5_1(lines: Lines) -> Result<String> {
    let (mut stacks, orders) = parse(lines)?;
    for order in orders {
        stacks.execute_order_9000(&order)?;
    }
    Ok(stacks.read_top())
}

pub fn day_5_2(lines: Lines) -> Result<String> {
    let (mut stacks, orders) = parse(lines)?;
    for order in orders {
        stacks.execute_order_9001(&order)?;
    }
    Ok(stacks.read_top())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::aoc_test;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    const INPUT_2: &str = "[A]
[B] [C]
[D] [E] [F]
 1   2   3

move 2 from 1 to 3
move 1 from 1 to 3
move 1 from 3 to 1
move 2 from 3 to 1
";

    #[test]
    fn test_day_5_1() {
        aoc_test(INPUT, day_5_1, "CMZ".to_string());
    }

    #[test]
    fn test_day_5_2() {
        aoc_test(INPUT, day_5_2, "MCD".to_string());
    }

    #[test]
    fn my_test() {
        aoc_test(INPUT_2, day_5_1, "ACF".to_string());
    }

    #[test]
    fn order() {
        assert_eq!(
            parse_order("move 1 from 2 to 1").unwrap(),
            Order {
                how_many: 1,
                from: 1,
                to: 0
            }
        );
    }
}
