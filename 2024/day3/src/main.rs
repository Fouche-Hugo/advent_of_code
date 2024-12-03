use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

static MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap());
static REGEX_PART_2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(mul\(([0-9]*),([0-9]*)\))|(do\(\))|(don't\(\))").unwrap());

#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    input_file: PathBuf,
    #[clap(long)]
    part2: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let result = if args.part2 {
        let instructions = parse_input_part2(&args.input_file)?;
        part2(instructions)
    } else {
        let muls = parse_input(&args.input_file)?;
        part1(muls)
    };
    println!("{result}");
    Ok(())
}

fn part1(muls: Vec<(i64, i64)>) -> i64 {
    muls.into_iter().fold(0, |acc, val| acc + val.0 * val.1)
}

fn part2(instructions: Vec<Instruction>) -> i64 {
    let mut active = true;

    let mut total = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
            Instruction::Mul(a, b) => {
                if active {
                    total += a * b;
                }
            }
        }
    }
    total
}

fn parse_input(input_file: &Path) -> Result<Vec<(i64, i64)>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let matches = MUL_REGEX
        .captures_iter(&file_input)
        .into_iter()
        .map(|l| {
            (
                l.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                l.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            )
        })
        .collect();

    Ok(matches)
}

enum Instruction {
    Do,
    Dont,
    Mul(i64, i64),
}

fn parse_input_part2(input_file: &Path) -> Result<Vec<Instruction>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let matches = REGEX_PART_2
        .captures_iter(&file_input)
        .into_iter()
        .map(|l| {
            let first_part = l.get(0).unwrap().as_str();
            if first_part == "do()" {
                Instruction::Do
            } else if first_part == "don't()" {
                Instruction::Dont
            } else {
                Instruction::Mul(
                    l.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                    l.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                )
            }
        })
        .collect();

    Ok(matches)
}
