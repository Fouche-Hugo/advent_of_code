use anyhow::Result;
use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    collections::HashSet,
    hash::Hash,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
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

fn part1(map: Vec<Vec<char>>) -> usize {
    let (mut position, mut direction) = get_guard_position(&map);
    let mut positions = HashSet::new();

    while map
        .get(position.y as usize)
        .is_some_and(|line| line.get(position.x as usize).is_some())
    {
        positions.insert(position);
        let next_position = match direction {
            GuardDirection::North => Position {
                x: position.x,
                y: position.y - 1,
            },
            GuardDirection::South => Position {
                x: position.x,
                y: position.y + 1,
            },
            GuardDirection::East => Position {
                x: position.x + 1,
                y: position.y,
            },
            GuardDirection::West => Position {
                x: position.x - 1,
                y: position.y,
            },
        };

        if map
            .get(next_position.y as usize)
            .and_then(|line| line.get(next_position.x as usize))
            .is_some_and(|char| *char == '#')
        {
            direction.rotate();
        }

        position.update(direction);
    }

    positions.len()
}

fn get_guard_position(map: &[Vec<char>]) -> (Position, GuardDirection) {
    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            let guard_direction: Result<GuardDirection, String> = (*ch).try_into();

            if let Ok(direction) = guard_direction {
                return (
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                    direction,
                );
            }
        }
    }

    panic!("Failed to get guard position");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn update(&mut self, direction: GuardDirection) {
        match direction {
            GuardDirection::North => self.y -= 1,
            GuardDirection::South => self.y += 1,
            GuardDirection::East => self.x += 1,
            GuardDirection::West => self.x -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GuardDirection {
    North,
    South,
    East,
    West,
}

impl GuardDirection {
    pub fn rotate(&mut self) {
        *self = match self {
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::North => Self::East,
        }
    }
}

impl TryFrom<char> for GuardDirection {
    type Error = String;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        let direction = match value {
            '^' => GuardDirection::North,
            '>' => GuardDirection::East,
            '<' => GuardDirection::West,
            'v' => GuardDirection::South,
            _ => return Err("Failed to parse guard direction".to_string()),
        };

        Ok(direction)
    }
}

fn part2(map: Vec<Vec<char>>) -> usize {
    let total_loop_positions = Arc::new(Mutex::new(0));

    (0..map.len()).into_par_iter().for_each(|y| {
        (0..map[y].len()).into_par_iter().for_each(|x| {
            if map[y][x] == '.'
                && try_stuck_in_loop(
                    &map,
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                )
            {
                *total_loop_positions.lock().unwrap() += 1;
            }
        })
    });

    let total_loop_positions = total_loop_positions.lock().unwrap();

    *total_loop_positions
}

fn try_stuck_in_loop(map: &[Vec<char>], new_obstacle: Position) -> bool {
    let mut map: Vec<Vec<char>> = map.iter().map(|line| line.clone()).collect();
    map[new_obstacle.y as usize][new_obstacle.x as usize] = '#';

    let (mut position, mut direction) = get_guard_position(&map);
    map[position.y as usize][position.x as usize] = '.';
    let mut positions_with_dir: HashSet<(Position, GuardDirection)> = HashSet::new();

    while map
        .get(position.y as usize)
        .is_some_and(|line| line.get(position.x as usize).is_some())
    {
        if positions_with_dir.contains(&(position, direction)) {
            return true;
        }

        positions_with_dir.insert((position, direction));
        let next_position = match direction {
            GuardDirection::North => Position {
                x: position.x,
                y: position.y - 1,
            },
            GuardDirection::South => Position {
                x: position.x,
                y: position.y + 1,
            },
            GuardDirection::East => Position {
                x: position.x + 1,
                y: position.y,
            },
            GuardDirection::West => Position {
                x: position.x - 1,
                y: position.y,
            },
        };

        if map
            .get(next_position.y as usize)
            .and_then(|line| line.get(next_position.x as usize))
            .is_some_and(|char| *char == '#')
        {
            direction.rotate();
        } else {
            position.update(direction);
        }
    }

    false
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<char>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}
