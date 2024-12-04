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
    let chars = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(chars)
    } else {
        part1(chars)
    };
    println!("{result}");
    Ok(())
}

fn part1(chars: Vec<Vec<char>>) -> usize {
    let mut total_xmas = 0;
    for (i, xchs) in chars.iter().enumerate() {
        for (j, ch) in xchs.iter().enumerate() {
            if *ch == 'X' {
                total_xmas += check_xmas(&chars, i, j)
            }
        }
    }

    total_xmas
}

const XMAS: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
fn check_xmas(chars: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut xmas_amount = 0;

    for (dir_x, dir_y) in DIRECTIONS {
        let mut full_xmas = true;
        for k in 0..XMAS.len() {
            if let Some(char) = chars
                .get((i as isize + dir_y * k as isize) as usize)
                .and_then(|xchars| xchars.get((j as isize + dir_x * k as isize) as usize))
            {
                if *char != XMAS.chars().nth(k).unwrap() {
                    full_xmas = false;
                    break;
                }
            } else {
                full_xmas = false;
                break;
            }
        }

        if full_xmas {
            xmas_amount += 1;
        }
    }

    xmas_amount
}

fn part2(chars: Vec<Vec<char>>) -> usize {
    let mut total_x_mas = 0;
    for (i, xchs) in chars.iter().enumerate() {
        for (j, ch) in xchs.iter().enumerate() {
            if *ch == 'A' && check_x_mas(&chars, i, j) {
                total_x_mas += 1
            }
        }
    }

    total_x_mas
}

const MAS_DIR: [(isize, isize); 2] = [(1, 1), (-1, 1)];
const MAS_DIR_COEFS: [isize; 2] = [1, -1];

fn check_x_mas(chars: &[Vec<char>], i: usize, j: usize) -> bool {
    let both_mas_correct = true;
    for (x_dir, y_dir) in MAS_DIR {
        let mut letters = vec![];
        for coef in MAS_DIR_COEFS {
            if let Some(ch) = chars
                .get((i as isize + coef * y_dir) as usize)
                .and_then(|xchars| xchars.get((j as isize + coef * x_dir) as usize))
            {
                letters.push(ch);
            }
        }
        if !letters.contains(&&'S') || !letters.contains(&&'M') {
            return false;
        }
    }

    both_mas_correct
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<char>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}
