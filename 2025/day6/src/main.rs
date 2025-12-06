use anyhow::Result;
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

fn main() -> Result<()> {
    let args = Args::parse();

    let result = if args.part2 {
        part2(&args.input_file)
    } else {
        part1(&args.input_file)
    };
    println!("{result}");
    Ok(())
}

fn compute_problems(problems: Vec<Problem>) -> usize {
    problems
        .into_iter()
        .map(|problem| match problem.operation {
            Operation::Add => problem.numbers.iter().sum(),
            Operation::Multiply => problem.numbers.iter().fold(1, |acc, number| acc * number),
        })
        .sum()
}

fn part1(input_file: &Path) -> usize {
    let problems = parse_input_part1(input_file).unwrap();

    compute_problems(problems)
}

fn part2(input_file: &Path) -> usize {
    let problems = parse_input_part2(input_file).unwrap();

    compute_problems(problems)
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let operation = match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => return Err(anyhow::anyhow!("unable to parse operation from {s}")),
        };

        Ok(operation)
    }
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

fn parse_input_part1(input_file: &Path) -> Result<Vec<Problem>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let mut lines = file_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let line_numbers: Vec<Vec<usize>> = lines
        .by_ref()
        .take_while(|line| {
            let first_char = line.chars().next().unwrap();

            first_char != '*' && first_char != '+'
        })
        .map(|line| {
            line.split(' ')
                .filter(|part| !part.is_empty())
                .map(|part| part.parse().unwrap())
                .collect()
        })
        .collect();

    let operations: Vec<Operation> = file_input
        .lines()
        .rev()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .unwrap()
        .split(' ')
        .filter(|part| !part.is_empty())
        .map(|part| Operation::from_str(part).unwrap())
        .collect();

    let mut problems = vec![];

    for (i, operation) in operations.into_iter().enumerate() {
        let numbers = line_numbers.iter().map(|line| line[i]).collect();

        problems.push(Problem { numbers, operation })
    }

    Ok(problems)
}

fn parse_input_part2(input_file: &Path) -> Result<Vec<Problem>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let max_line_len = file_input.lines().map(|line| line.len()).max().unwrap();
    let mut empty_col_indexes = vec![];

    'outer: for col_index in 0..max_line_len {
        for line in file_input.lines() {
            if line.chars().nth(col_index).unwrap() != ' ' {
                continue 'outer;
            }
        }
        empty_col_indexes.push(col_index);
    }

    let mut col_ranges = vec![];

    for (i, empty_col) in empty_col_indexes.iter().enumerate() {
        if i == 0 {
            col_ranges.push(0..*empty_col);
            continue;
        }

        col_ranges.push(empty_col_indexes[i - 1] + 1..*empty_col);
        if i == empty_col_indexes.len() - 1 {
            col_ranges.push(empty_col + 1..max_line_len);
        }
    }

    let mut problems = vec![];
    for range in col_ranges {
        let start = range.start;

        let mut cols: Vec<_> = vec![];
        for col_index in range {
            cols.push(
                file_input
                    .lines()
                    .map(move |line| line.chars().nth(col_index).unwrap()),
            );
        }

        let numbers: Vec<usize> = cols
            .into_iter()
            .map(|col| {
                col.into_iter()
                    .filter(|&c| c != ' ' && c != '+' && c != '*')
                    .fold(String::new(), |mut acc, c| {
                        acc.push(c);
                        acc
                    })
                    .parse()
                    .unwrap()
            })
            .collect();

        let operation = Operation::from_str(
            &file_input
                .lines()
                .rev()
                .find(|line| !line.is_empty())
                .unwrap()
                .chars()
                .nth(start)
                .unwrap()
                .to_string(),
        )
        .unwrap();

        problems.push(Problem { numbers, operation });
    }

    Ok(problems)
}
