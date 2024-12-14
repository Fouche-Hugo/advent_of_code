use anyhow::Result;
use clap::Parser;
use std::{
    cmp::min,
    collections::HashSet,
    ops::{Add, AddAssign, Mul, Sub},
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
    let map = parse_input(&args.input_file)?;

    let result = if args.part2 { part2(map) } else { part1(map) };
    println!("{result}");
    Ok(())
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    x: usize,
    y: usize,
}

impl Mul<usize> for Move {
    type Output = Move;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Move> for Move {
    type Output = Move;

    fn add(self, rhs: Move) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Move> for Position {
    type Output = Position;

    fn add(self, rhs: Move) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Move> for Position {
    fn add_assign(&mut self, rhs: Move) {
        *self = *self + rhs
    }
}

#[derive(Debug)]
struct Machine {
    a: Move,
    b: Move,
    target: Position,
}

fn machine_minimum_tokens(machine: &Machine, max_presses: usize) -> Option<usize> {
    let mut min_tokens = None;
    let initial_position = Position { x: 0, y: 0 };

    for i in 0..max_presses {
        let a_tokens = i * 3;
        if min_tokens.is_some_and(|min_tokens| a_tokens > min_tokens) {
            break;
        }

        for j in 0..max_presses {
            let b_tokens = j;
            if min_tokens.is_some_and(|min_tokens| a_tokens + b_tokens > min_tokens) {
                continue;
            }

            let position = initial_position + machine.a * i + machine.b * j;

            if position == machine.target {
                min_tokens = Some(a_tokens + b_tokens);
            }

            if position.x > machine.target.x || position.y > machine.target.y {
                break;
            }
        }
    }

    min_tokens
}

// fn machine_minimum_tokens(machine: &Machine, current: Position) -> Option<usize> {
//     let current_a = current + machine.a;
//     let current_b = current + machine.b;
//     if current_a == machine.target {
//         Some(3)
//     } else if current_b == machine.target {
//         Some(1)
//     } else if current_a.x > machine.target.x
//         || current_a.y > machine.target.y
//         || current_b.x > machine.target.x
//         || current_b.y > machine.target.y
//     {
//         None
//     } else {
//         let minimum_tokens_a = machine_minimum_tokens(machine, current_a);
//         let minimum_tokens_b = machine_minimum_tokens(machine, current_b);

//         match (minimum_tokens_a, minimum_tokens_b) {
//             (Some(a_val), Some(b_val)) => Some(min(3 + a_val, 1 + b_val)),
//             (Some(a_val), None) => Some(3 + a_val),
//             (None, Some(b_val)) => Some(1 + b_val),
//             (None, None) => None,
//         }
//     }
// }

fn part1(machines: Vec<Machine>) -> usize {
    let mut total = 0;

    for machine in machines {
        if let Some(tokens) = machine_minimum_tokens(&machine, 100) {
            total += tokens;
        }
    }

    total
}

fn part2(machines: Vec<Machine>) -> usize {
    let mut total = 0;

    for machine in machines {
        if let Some(tokens) = machine_minimum_tokens(&machine, 100) {
            // // check if multiple of 1000000000
            // let mut tokens_1000000000 = None;
            // for i in 1..9 {
            //     let machine_i = Machine {
            //         a: machine.a,
            //         b: machine.b,
            //         target: Position {
            //             x: 10_usize.pow(i),
            //             y: 10_usize.pow(i),
            //         },
            //     };

            //     if let Some(min_tokens_i) = machine_minimum_tokens(&machine_i, 100) {
            //         tokens_1000000000 = Some(min_tokens_i * 10_usize.pow((9 - i) as u32));
            //         break;
            //     }
            // }

            // if let Some(tokens_1000000000) = tokens_1000000000 {
            //     total += tokens + tokens_1000000000;
            // }

            // if format!("1000000000{}", machine.target).parse().unwrap() {}
        }
    }

    total
}

fn parse_input(input_file: &Path) -> Result<Vec<Machine>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let mut machines = vec![];

    for machine_lines in file_input.lines().collect::<Vec<&str>>().chunks(4) {
        let a_x_index = machine_lines[0].find('X').unwrap();
        let mut a_button_split =
            machine_lines[0][a_x_index + 2..machine_lines[0].len()].split(", Y+");
        let a_x = a_button_split.next().unwrap();
        let a_y = a_button_split.next().unwrap();

        let b_x_index = machine_lines[1].find('X').unwrap();
        let mut b_button_split =
            machine_lines[1][b_x_index + 2..machine_lines[1].len()].split(", Y+");
        let b_x = b_button_split.next().unwrap();
        let b_y = b_button_split.next().unwrap();

        let prize_x_index = machine_lines[2].find('X').unwrap();
        let mut prize_button_split =
            machine_lines[2][prize_x_index + 2..machine_lines[2].len()].split(", Y=");
        let prize_x = prize_button_split.next().unwrap();
        let prize_y = prize_button_split.next().unwrap();

        machines.push(Machine {
            a: Move {
                x: a_x.parse().unwrap(),
                y: a_y.parse().unwrap(),
            },
            b: Move {
                x: b_x.parse().unwrap(),
                y: b_y.parse().unwrap(),
            },
            target: Position {
                x: prize_x.parse().unwrap(),
                y: prize_y.parse().unwrap(),
            },
        });
    }

    Ok(machines)
}
