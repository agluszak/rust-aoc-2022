use crate::Lines;

fn common(lines: Lines) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut elf_sum = 0;
    for line in lines {
        if let Ok(num) = line.parse::<i32>() {
            elf_sum += num;
        } else {
            elves.push(elf_sum);
            elf_sum = 0;
        }
    }
    elves.push(elf_sum);

    elves.sort();
    elves.reverse();
    elves
}

pub fn day_1_1(lines: Lines) -> anyhow::Result<i32> {
    let elves = common(lines);
    Ok(elves.iter().take(1).sum())
}

pub fn day_1_2(lines: Lines) -> anyhow::Result<i32> {
    let elves = common(lines);
    Ok(elves.iter().take(3).sum())
}
