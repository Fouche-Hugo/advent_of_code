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

fn part1(input: Vec<Vec<Character>>) -> usize {
    let mut total = 0;

    for i in 0..input.len() {
        let line = &input[i];
        for j in 0..line.len() {
            match line[j] {
                Character::Dot => continue,
                Character::Roll => {
                    // check adjacent
                    let min_x = j.saturating_sub(1);
                    let max_x = (j + 1).min(line.len() - 1);

                    let min_y = i.saturating_sub(1);
                    let max_y = (i + 1).min(input.len() - 1);

                    let adjacent_rolls: usize = (min_x..=max_x)
                        .into_iter()
                        .map(|x| {
                            (min_y..=max_y)
                                .into_iter()
                                .filter(|y| (x != j || *y != i) && input[*y][x] == Character::Roll)
                                .count()
                        })
                        .sum();

                    if adjacent_rolls < 4 {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn part2(mut input: Vec<Vec<Character>>) -> usize {
    let mut total = 0;

    let y_size = input.len();
    let x_size = input[0].len();

    let mut was_updated = true;

    while was_updated {
        was_updated = false;

        for i in 0..y_size {
            for j in 0..x_size {
                match input[i][j] {
                    Character::Dot => continue,
                    Character::Roll => {
                        // check adjacent
                        let min_x = j.saturating_sub(1);
                        let max_x = (j + 1).min(x_size - 1);

                        let min_y = i.saturating_sub(1);
                        let max_y = (i + 1).min(y_size - 1);

                        let adjacent_rolls: usize = (min_x..=max_x)
                            .into_iter()
                            .map(|x| {
                                (min_y..=max_y)
                                    .into_iter()
                                    .filter(|y| {
                                        (x != j || *y != i) && input[*y][x] == Character::Roll
                                    })
                                    .count()
                            })
                            .sum();

                        if adjacent_rolls < 4 {
                            input[i][j] = Character::Dot;
                            was_updated = true;
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    total
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Character {
    Roll,
    Dot,
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<Character>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let ranges = file_input
        .lines()
        .into_iter()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Character::Roll,
                    '.' => Character::Dot,
                    _ => panic!("input should only have @ or ."),
                })
                .collect()
        })
        .collect();

    Ok(ranges)
}
