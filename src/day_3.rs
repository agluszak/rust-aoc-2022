use crate::Lines;
use anyhow::bail;
use itertools::Itertools;
use std::collections::BTreeSet;

fn priority(c: char) -> i32 {
    if c.is_ascii_alphabetic() {
        if c.is_ascii_uppercase() {
            27 + c as i32 - 'A' as i32
        } else {
            1 + c as i32 - 'a' as i32
        }
    } else {
        0
    }
}

pub fn day_3_1(lines: Lines) -> Result<i32, anyhow::Error> {
    let mut sum = 0;
    for line in lines {
        if line.len() % 2 != 0 {
            bail!("Invalid line length: {}", line.len());
        }
        let first_half = &line[0..line.len() / 2].chars().collect::<BTreeSet<_>>();
        let second_half = &line[line.len() / 2..].chars().collect::<BTreeSet<_>>();

        let in_both = first_half
            .intersection(second_half)
            .collect::<Vec<_>>()
            .first()
            .copied()
            .copied();

        if let Some(c) = in_both {
            sum += priority(c);
        } else {
            bail!("No common character in line: {}", line);
        }
    }

    Ok(sum)
}

pub fn day_3_2(lines: Lines) -> Result<i32, anyhow::Error> {
    let mut sum = 0;
    for (elf1, elf2, elf3) in lines.tuples() {
        let set1 = elf1.chars().collect::<BTreeSet<_>>();
        let set2 = elf2.chars().collect::<BTreeSet<_>>();
        let set3 = elf3.chars().collect::<BTreeSet<_>>();

        let in_all = set1
            .intersection(&set2)
            .cloned()
            .collect::<BTreeSet<_>>()
            .intersection(&set3)
            .collect::<Vec<_>>()
            .first()
            .copied()
            .copied();

        if let Some(c) = in_all {
            sum += priority(c);
        } else {
            bail!("No common character in line: {}, {}, {}", elf1, elf2, elf3);
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lines;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_test() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn test_day_3_1() {
        let input = Lines::from_text(INPUT);
        assert_eq!(day_3_1(input).unwrap(), 157);
    }

    #[test]
    fn test_day_3_2() {
        let input = Lines::from_text(INPUT);
        assert_eq!(day_3_2(input).unwrap(), 70);
    }
}
