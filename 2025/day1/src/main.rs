use anyhow::{Result, anyhow};
use clap::Parser;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    input_file: PathBuf,
    #[clap(long)]
    part2: bool,
}

const DIAL_START: usize = 50;

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

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let direction = match s {
            "L" => Self::Left,
            "R" => Self::Right,
            _ => return Err(anyhow!("failed to parse direction: {s}")),
        };
        Ok(direction)
    }
}

fn part1(instructions: Vec<Instruction>) -> usize {
    let mut dial_pointing_0 = 0;
    let mut current_dial = DIAL_START;

    for instruction in instructions {
        let new_dial = next_dial_position(current_dial, instruction);
        assert!(
            new_dial < 100,
            "current: {current_dial}, new: {new_dial}: instruction: {instruction:?}"
        );

        if new_dial == 0 {
            dial_pointing_0 += 1;
        }
        current_dial = new_dial
    }

    dial_pointing_0
}

fn next_dial_position(dial_current: usize, instruction: Instruction) -> usize {
    let instruction_amount = instruction.amount % 100;

    match instruction.direction {
        Direction::Left => {
            if instruction_amount > dial_current {
                100 - (instruction_amount - dial_current)
            } else {
                dial_current - instruction_amount
            }
        }
        Direction::Right => (dial_current + instruction_amount) % 100,
    }
}

fn part2(instructions: Vec<Instruction>) -> usize {
    let mut dial_pointing_0 = 0;
    let mut current_dial = DIAL_START;

    for instruction in instructions {
        let new_dial = next_dial_position(current_dial, instruction);
        assert!(
            new_dial < 100,
            "current: {current_dial}, new: {new_dial}: instruction: {instruction:?}"
        );

        dial_pointing_0 += part2_dial_passed_through_0(current_dial, instruction);
        current_dial = new_dial
    }

    dial_pointing_0
}

fn part2_dial_passed_through_0(current_dial: usize, instruction: Instruction) -> usize {
    let mut multiplier = instruction.amount / 100;

    if current_dial != 0 {
        if let Direction::Left = instruction.direction
            && instruction.amount % 100 >= current_dial
        {
            multiplier += 1;
        } else if let Direction::Right = instruction.direction
            && instruction.amount % 100 + current_dial >= 100
        {
            multiplier += 1
        }
    }

    match instruction.direction {
        Direction::Left => {
            if current_dial <= instruction.amount{
                multiplier
            } else {
                0
            }
        }
        Direction::Right => {
            if current_dial + instruction.amount >= 100 {
                multiplier
            } else {
                0
            }
        }
    }
}

fn parse_input(input_file: &Path) -> Result<Vec<Instruction>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let instructions = file_input
        .lines()
        .into_iter()
        .filter_map(|line| {
            let line = line.trim();

            if line.len() > 0 { Some(line) } else { None }
        })
        .map(|line| {
            let direction: Direction = line[0..1].parse()?;
            let amount: usize = line[1..].parse()?;
            Ok(Instruction { direction, amount })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(instructions)
}
