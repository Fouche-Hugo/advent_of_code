use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    input_file: PathBuf,
    #[clap(long)]
    part2: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let lines = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(lines)
    } else {
        part1(lines)
    };
    println!("{result}");
    Ok(())
}

fn part1(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .map(|line| {
            let chars = line
                .chars()
                .map(|c| -> u8 { c.to_string().parse().unwrap() })
                .collect::<Vec<_>>();

            let max = chars.iter().max().unwrap();

            let first_max_index = chars.iter().position(|c| c == max).unwrap();

            if first_max_index == chars.len() - 1 {
                let second_max = chars.iter().filter(|c| *c != max).max().unwrap();

                return format!("{second_max}{max}").parse().unwrap();
            }

            let second_max = chars[first_max_index + 1..].iter().max().unwrap();

            let value: usize = format!("{max}{second_max}").parse().unwrap();
            value
        })
        .fold(0, |acc, value| acc + value)
}

fn part2(lines: Vec<String>) -> usize {
    lines
        .into_iter()
        .map(|line| {
            let chars = line
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>();

            let mut joltage = vec![];
            let mut current_min_index = 0;

            for i in (0..12).into_iter().rev() {
                let chars = &chars[current_min_index..chars.len() - i];
                let max = chars.iter().max().unwrap();
                let max_index = chars.iter().position(|c| c == max).unwrap();

                joltage.push(max);
                current_min_index = 1 + current_min_index + max_index;
            }

            let joltage = joltage
                .into_iter()
                .map(|c| c.to_string())
                .fold(String::new(), |acc, value| acc + &value)
                .parse::<usize>()
                .unwrap();

            joltage
        })
        .sum()
}

fn parse_input(input_file: &Path) -> Result<Vec<String>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let ranges = file_input
        .lines()
        .into_iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(ranges)
}
