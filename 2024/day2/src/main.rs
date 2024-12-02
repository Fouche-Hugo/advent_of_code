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
    let reports = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(reports)
    } else {
        part1(reports)
    };
    println!("{result}");
    Ok(())
}

fn part1(reports: Vec<Vec<i64>>) -> i64 {
    let mut safe_reports = 0;
    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn part2(reports: Vec<Vec<i64>>) -> i64 {
    let mut safe_reports = 0;
    let mut unsafe_reports = vec![];
    for report in reports {
        if is_report_safe(&report) {
            safe_reports += 1;
        } else {
            unsafe_reports.push(report)
        }
    }

    // new "safe" report from unsafe ones
    for report in unsafe_reports {
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            if is_report_safe(&report) {
                safe_reports += 1;
                break;
            }
        }
    }

    safe_reports
}

fn is_report_safe(report: &[i64]) -> bool {
    let increasing = report[1] - report[0] > 0;
    for i in 1..report.len() {
        if increasing && report[i] < report[i - 1] {
            return false;
        }
        if !increasing && report[i] > report[i - 1] {
            return false;
        }
        let diff = (report[i] - report[i - 1]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<i64>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| {
            line.split(" ")
                .filter_map(|part| part.parse().ok())
                .collect()
        })
        .filter(|line: &Vec<i64>| !line.is_empty())
        .collect())
}
