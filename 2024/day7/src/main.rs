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
    let equations = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(equations)
    } else {
        part1(equations)
    };
    println!("{result}");
    Ok(())
}

fn part1(equations: Vec<Equation>) -> usize {
    let mut total = 0;

    for equation in equations {
        if is_equation_valid(&equation) {
            total += equation.target
        }
    }

    total
}

fn is_equation_valid(equation: &Equation) -> bool {
    if equation.values.len() == 1 {
        equation.target == equation.values[0]
    } else if equation.values.len() == 2 {
        equation.target == equation.values[0] + equation.values[1]
            || equation.target == equation.values[0] * equation.values[1]
    } else {
        let add_equation = Equation {
            target: equation.target,
            values: [equation.values[0] + equation.values[1]]
                .into_iter()
                .chain(equation.values.iter().skip(2).map(|value| *value))
                .collect(),
        };

        let mul_equation = Equation {
            target: equation.target,
            values: [equation.values[0] * equation.values[1]]
                .into_iter()
                .chain(equation.values.iter().skip(2).map(|value| *value))
                .collect(),
        };

        is_equation_valid(&add_equation) || is_equation_valid(&mul_equation)
    }
}

fn part2(equations: Vec<Equation>) -> usize {
    let mut total = 0;

    for equation in equations {
        if is_equation_valid_part2(&equation) {
            total += equation.target
        }
    }

    total
}

fn is_equation_valid_part2(equation: &Equation) -> bool {
    if equation.values.len() == 1 {
        equation.target == equation.values[0]
    } else if equation.values.len() == 2 {
        equation.target == equation.values[0] + equation.values[1]
            || equation.target == equation.values[0] * equation.values[1]
            || equation.target
                == format!("{}{}", equation.values[0], equation.values[1])
                    .parse()
                    .unwrap()
    } else {
        let add_equation = Equation {
            target: equation.target,
            values: [equation.values[0] + equation.values[1]]
                .into_iter()
                .chain(equation.values.iter().skip(2).map(|value| *value))
                .collect(),
        };

        let mul_equation = Equation {
            target: equation.target,
            values: [equation.values[0] * equation.values[1]]
                .into_iter()
                .chain(equation.values.iter().skip(2).map(|value| *value))
                .collect(),
        };

        let concat_equation = Equation {
            target: equation.target,
            values: [format!("{}{}", equation.values[0], equation.values[1])
                .parse()
                .unwrap()]
            .into_iter()
            .chain(equation.values.iter().skip(2).map(|value| *value))
            .collect(),
        };

        is_equation_valid_part2(&add_equation)
            || is_equation_valid_part2(&mul_equation)
            || is_equation_valid_part2(&concat_equation)
    }
}

#[derive(Debug)]
struct Equation {
    target: usize,
    values: Vec<usize>,
}

fn parse_input(input_file: &Path) -> Result<Vec<Equation>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| -> Result<Equation, ParseIntError> {
            let line_splitted: Vec<&str> = line.split(":").collect();

            let target: usize = line_splitted[0].parse()?;

            let values: Vec<usize> = line_splitted[1]
                .split(" ")
                .filter(|value| !value.is_empty())
                .map(|value| value.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?;

            Ok(Equation { target, values })
        })
        .collect::<Result<Vec<Equation>, ParseIntError>>()?)
}
