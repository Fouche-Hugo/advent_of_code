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
    let stones = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(stones)
    } else {
        part1(stones)
    };
    println!("{result}");
    Ok(())
}

fn part1(stones: Vec<usize>) -> usize {
    let mut stones = stones;
    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.len()
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = Vec::with_capacity(stones.len());

    for stone in stones {
        new_stones.extend(compute_stone(stone));
    }

    new_stones
}

#[cached::proc_macro::cached]
fn compute_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            vec![
                stone_str[0..stone_str.len() / 2].parse().unwrap(),
                stone_str[stone_str.len() / 2..stone_str.len()]
                    .parse()
                    .unwrap(),
            ]
        } else {
            vec![stone * 2024]
        }
    }
}

#[cached::proc_macro::cached]
fn compute_stone_n_times(stone: usize, n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let stones = compute_stone(stone);
        stones
            .into_iter()
            .map(|stone| compute_stone_n_times(stone, n - 1))
            .sum()
    }
}

fn part2(stones: Vec<usize>) -> usize {
    let mut total = 0;

    for stone in stones {
        total += compute_stone_n_times(stone, 75);
    }

    total
}

fn parse_input(input_file: &Path) -> Result<Vec<usize>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .split(" ")
        .filter(|part| !part.is_empty())
        .map(|part| part.parse().unwrap())
        .collect())
}
