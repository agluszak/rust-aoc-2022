use crate::Lines;
use anyhow::Result;
use regex::Regex;

fn common(lines: Lines, predicate: impl Fn(i32, i32, i32, i32) -> bool) -> Result<i32> {
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;

    let mut count = 0;
    for line in lines {
        let captures = regex
            .captures(&line)
            .ok_or_else(|| anyhow::anyhow!("Invalid line: {}", line))?;
        let (x1, x2, y1, y2) = (
            captures[1].parse::<i32>()?,
            captures[2].parse::<i32>()?,
            captures[3].parse::<i32>()?,
            captures[4].parse::<i32>()?,
        );
        if predicate(x1, x2, y1, y2) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn day_4_1(lines: Lines) -> anyhow::Result<i32> {
    // check if one is contained in the other
    common(lines, |x1, x2, y1, y2| {
        (x1 <= y1 && y2 <= x2) || (y1 <= x1 && x2 <= y2)
    })
}

pub fn day_4_2(lines: Lines) -> anyhow::Result<i32> {
    // check if there is *any* overlap
    let predicate = |x1, x2, y1, y2| {
        (x1 <= y1 && y1 <= x2)
            || (y1 <= x1 && x1 <= y2)
            || (x1 <= y2 && y2 <= x2)
            || (y1 <= x2 && x2 <= y2)
            || (x1 == y1 && x2 == y2)
    };
    common(lines, predicate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::aoc_test;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn day_4_1_test() {
        aoc_test(INPUT, day_4_1, 2);
    }

    #[test]
    fn day_4_2_test() {
        aoc_test(INPUT, day_4_2, 4);
    }
}
