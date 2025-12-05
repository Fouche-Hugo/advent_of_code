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
    let (ranges, ingredients) = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(ranges)
    } else {
        part1(ranges, ingredients)
    };
    println!("{result}");
    Ok(())
}

fn part1(ranges: Vec<RangeInclusive<usize>>, ingredients: Vec<usize>) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn part2(mut ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut update = true;

    while update {
        update = false;

        'outer: for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                let merged_range = merge_ranges(&ranges[i], &ranges[j]);

                if let Some(merged_range) = merged_range {
                    ranges[i] = merged_range;
                    ranges.remove(j);
                    update = true;
                    break 'outer;
                }
            }
        }
    }

    ranges.into_iter().map(|range| range.count()).sum()
}

fn merge_ranges(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if a.start() <= b.end() && b.start() <= a.end() {
        let start = a.start().min(b.start());
        let end = a.end().max(b.end());
        Some(*start..=*end)
    } else {
        None
    }
}

fn parse_input(input_file: &Path) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let file_input = std::fs::read_to_string(input_file)?;

    let mut lines = file_input.lines();
    let ranges = lines
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let mut pattern = line.split('-');
            pattern.next().unwrap().parse().unwrap()..=pattern.next().unwrap().parse().unwrap()
        })
        .collect();

    let ingredients = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    Ok((ranges, ingredients))
}
