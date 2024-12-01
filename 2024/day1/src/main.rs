use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashMap,
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
    let (list1, list2) = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(list1, list2)
    } else {
        part1(list1, list2)
    };
    println!("{result}");
    Ok(())
}

fn part1(mut list1: Vec<i64>, mut list2: Vec<i64>) -> i64 {
    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2)
        .fold(0, |acc, (list1_value, list2_value)| {
            acc + (list1_value - list2_value).abs()
        })
}

fn part2(list1: Vec<i64>, list2: Vec<i64>) -> i64 {
    let list2_counts = list2.into_iter().fold(HashMap::new(), |mut acc, num| {
        acc.entry(num).and_modify(|e| *e += 1).or_insert(1);
        acc
    });

    list1.into_iter().fold(0, |acc, list1_value| {
        acc + list1_value * list2_counts.get(&list1_value).unwrap_or(&0)
    })
}

fn parse_input(input_file: &Path) -> Result<(Vec<i64>, Vec<i64>)> {
    let file_input = std::fs::read_to_string(input_file)?;
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in file_input.lines() {
        let data: Vec<i64> = line
            .split(" ")
            .filter_map(|line| line.parse().ok())
            .collect();
        list1.push(data[0]);
        list2.push(data[1]);
    }

    Ok((list1, list2))
}
