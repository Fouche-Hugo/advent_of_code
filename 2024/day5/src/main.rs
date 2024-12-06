use anyhow::Result;
use clap::Parser;
use std::{
    num::ParseIntError,
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
    let (rules, updates) = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(rules, updates)
    } else {
        part1(rules, updates)
    };
    println!("{result}");
    Ok(())
}

fn part1(rules: Vec<Rule>, updates: Vec<Update>) -> usize {
    let mut total = 0;
    for update in updates {
        if is_update_correct(&rules, &update) {
            total += update.page_numbers[update.page_numbers.len() / 2]
        }
    }

    total
}

fn is_update_correct(rules: &[Rule], update: &Update) -> bool {
    let page_numbers = &update.page_numbers;
    for rule in rules {
        match (
            page_numbers
                .iter()
                .position(|page_number| *page_number == rule.before),
            page_numbers
                .iter()
                .position(|page_number| *page_number == rule.after),
        ) {
            (Some(before), Some(after)) => {
                if before > after {
                    return false;
                }
            }
            _ => continue,
        }
    }

    true
}

fn part2(rules: Vec<Rule>, updates: Vec<Update>) -> usize {
    let mut total = 0;

    for update in updates {
        if !is_update_correct(&rules, &update) {
            let corrected_update = sort_incorrect_update(&rules, update);
            total += corrected_update.page_numbers[corrected_update.page_numbers.len() / 2];
        }
    }

    total
}

fn sort_incorrect_update(rules: &[Rule], update: Update) -> Update {
    let mut corrected_update = update;

    println!("");
    println!("Before sort: {corrected_update:?}");
    while !is_update_correct(rules, &corrected_update) {
        for rule in rules {
            match (
                corrected_update
                    .page_numbers
                    .iter()
                    .position(|page_number| *page_number == rule.before),
                corrected_update
                    .page_numbers
                    .iter()
                    .position(|page_number| *page_number == rule.after),
            ) {
                (Some(before_index), Some(after_index)) => {
                    if before_index > after_index {
                        corrected_update.page_numbers[before_index] = rule.after;
                        corrected_update.page_numbers[after_index] = rule.before;
                    }
                }
                _ => continue,
            }
        }
    }

    println!("After sort: {corrected_update:?}");

    corrected_update
}

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

#[derive(Debug)]
struct Update {
    page_numbers: Vec<usize>,
}

fn parse_input(input_file: &Path) -> Result<(Vec<Rule>, Vec<Update>)> {
    let file_input = std::fs::read_to_string(input_file)?;

    let mut rules = vec![];
    let mut updates = vec![];
    for line in file_input.lines() {
        if line.contains("|") {
            let line_splitted: Vec<&str> = line.split("|").into_iter().collect();
            let rule = Rule {
                before: line_splitted[0].trim().parse()?,
                after: line_splitted[1].trim().parse()?,
            };
            rules.push(rule);
        } else if line.contains(",") {
            let line_splitted: Vec<usize> = line
                .split(",")
                .into_iter()
                .map(|part| part.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?;
            let update = Update {
                page_numbers: line_splitted,
            };
            updates.push(update);
        }
    }

    Ok((rules, updates))
}
