use anyhow::Result;
use clap::Parser;
use std::{
    ops::RangeInclusive,
    path::{Path, PathBuf},
};

#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    input_file: PathBuf,
    #[clap(long)]
    part2: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let instructions = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(instructions)
    } else {
        part1(instructions)
    };
    println!("{result}");
    Ok(())
}

fn part1(ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut total = 0;
    for range in ranges {
        for id in range {
            if part1_is_invalid(id) {
                println!("invalid: {id}");
                total += id;
            }
        }
    }

    total
}

fn part1_is_invalid(original_id: usize) -> bool {
    let id = original_id.to_string();

    if original_id == 11 {
        println!("{original_id}: result: {}", id.len() % 2);
    }
    if id.len() % 2 != 0 {
        return false;
    }

    let part1 = &id[..id.len() / 2];
    let part2 = &id[id.len() / 2..];

    if original_id == 11 {
        println!("{original_id}: part1: {part1}, part2: {part2}");
    }

    if part1 == part2 {
        return true;
    }

    false
}

fn part2(ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut total = 0;
    for range in ranges {
        for id in range {
            if part2_is_invalid(id) {
                println!("invalid: {id}");
                total += id;
            }
        }
    }

    total
}

fn part2_is_invalid(original_id: usize) -> bool {
    let id = original_id.to_string();

    for i in 1..id.len() {
        if original_id == 11 {
            println!("{original_id}: result: {}", id.len() % i);
        }
        if id.len() % i != 0 {
            continue;
        }

        let first_part = &id[0..i];

        let mut all_part_equal = true;

        for j in (i..id.len()).into_iter().step_by(i) {
            if first_part != &id[j..j + i] {
                all_part_equal = false;
                break;
            }
        }

        if all_part_equal {
            return true;
        }
    }

    false
}

fn parse_input(input_file: &Path) -> Result<Vec<RangeInclusive<usize>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let ranges = file_input
        .lines()
        .into_iter()
        .fold(String::new(), |acc, line| acc + line)
        .split(",")
        .map(|pattern| {
            let mut pattern = pattern.trim().split("-");

            pattern.next().unwrap().parse().unwrap()..=pattern.next().unwrap().parse().unwrap()
        })
        .collect::<Vec<RangeInclusive<usize>>>();

    Ok(ranges)
}
