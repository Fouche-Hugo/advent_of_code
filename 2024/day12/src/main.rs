use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn checked_sub(&self, other: &Position) -> Option<Position> {
        Some(Self {
            x: self.x.checked_sub(other.x)?,
            y: self.y.checked_sub(other.y)?,
        })
    }
}

const NEIGHBOUR_POSITIVE: [Position; 2] = [Position { x: 1, y: 0 }, Position { x: 0, y: 1 }];
fn visit_area(
    map: &[Vec<char>],
    start: Position,
    mut visited_positions: HashSet<Position>,
) -> (usize, HashSet<Position>) {
    let current_plant = map[start.y][start.x];
    let mut neighbours = 0;
    let mut perimeter = 0;

    visited_positions.insert(start);

    // positive diff
    for diff in NEIGHBOUR_POSITIVE {
        let current_position = start + diff;
        if current_position.x < map[0].len()
            && current_position.y < map.len()
            && map[current_position.y][current_position.x] == current_plant
        {
            neighbours += 1;

            if !visited_positions.contains(&current_position) {
                let (current_perimeter, current_visited_positions) =
                    visit_area(map, current_position, visited_positions);
                perimeter += current_perimeter;
                visited_positions = current_visited_positions;
            }
        }
    }

    // negative diff
    for diff in NEIGHBOUR_POSITIVE {
        if let Some(current_position) = start.checked_sub(&diff) {
            if map[current_position.y][current_position.x] == current_plant {
                neighbours += 1;

                if !visited_positions.contains(&current_position) {
                    let (current_perimeter, current_visited_positions) =
                        visit_area(map, current_position, visited_positions);
                    perimeter += current_perimeter;
                    visited_positions = current_visited_positions;
                }
            }
        }
    }

    let plant_perimeter = 4 - neighbours;

    (plant_perimeter + perimeter, visited_positions)
}

fn part1(map: Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut visited_positions = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for x in 0..line.len() {
            let current_position = Position { x, y };
            if !visited_positions.contains(&current_position) {
                let (perimeter, current_visited_positions) =
                    visit_area(&map, current_position, HashSet::new());
                total += perimeter * current_visited_positions.len();
                visited_positions.extend(current_visited_positions);
            }
        }
    }

    total
}

fn visit_area_part2(
    map: &[Vec<char>],
    start: Position,
    mut visited_positions: HashSet<Position>,
) -> (usize, HashSet<Position>) {
    let current_plant = map[start.y][start.x];
    let mut pos_neighbours = vec![];
    let mut neg_neighbours = vec![];
    let mut nb_turns = 0;

    visited_positions.insert(start);

    // positive diff
    for diff in NEIGHBOUR_POSITIVE {
        let current_position = start + diff;
        if current_position.x < map[0].len()
            && current_position.y < map.len()
            && map[current_position.y][current_position.x] == current_plant
        {
            pos_neighbours.push(diff);

            if !visited_positions.contains(&current_position) {
                let (current_nb_turns, current_visited_positions) =
                    visit_area_part2(map, current_position, visited_positions);
                nb_turns += current_nb_turns;
                visited_positions = current_visited_positions;
            }
        }
    }

    // negative diff
    for diff in NEIGHBOUR_POSITIVE {
        if let Some(current_position) = start.checked_sub(&diff) {
            if map[current_position.y][current_position.x] == current_plant {
                neg_neighbours.push(diff);

                if !visited_positions.contains(&current_position) {
                    let (current_nb_turns, current_visited_positions) =
                        visit_area_part2(map, current_position, visited_positions);
                    nb_turns += current_nb_turns;
                    visited_positions = current_visited_positions;
                }
            }
        }
    }

    let plant_turns = if pos_neighbours.is_empty() && neg_neighbours.is_empty() {
        4
    } else if pos_neighbours.is_empty() && neg_neighbours.len() == 1 {
        2
    } else if neg_neighbours.is_empty() && pos_neighbours.len() == 1 {
        2
    } else if pos_neighbours.len() == 1 && neg_neighbours.len() == 1 {
        if pos_neighbours[0].x == neg_neighbours[0].x || pos_neighbours[0].y == neg_neighbours[0].y
        {
            0
        } else {
            // check if diagonals exists
            let neighbour1 = Position {
                x: start.x + pos_neighbours[0].x,
                y: start.y - neg_neighbours[0].y,
            };
            let neighbour2 = Position {
                x: start.x - neg_neighbours[0].x,
                y: start.y + pos_neighbours[0].y,
            };
            if map[neighbour1.y][neighbour1.x] == current_plant
                && map[neighbour2.y][neighbour2.x] == current_plant
            {
                1
            } else {
                2
            }
        }
    } else {
        1
    };

    (plant_turns + nb_turns, visited_positions)
}

fn part2(map: Vec<Vec<char>>) -> usize {
    let mut total = 0;

    let mut visited_positions = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for x in 0..line.len() {
            let current_position = Position { x, y };
            if !visited_positions.contains(&current_position) {
                let (turns, current_visited_positions) =
                    visit_area_part2(&map, current_position, HashSet::new());
                println!(
                    "region ({x}, {y}): {turns}, {}",
                    current_visited_positions.len()
                );
                total += turns * current_visited_positions.len();
                visited_positions.extend(current_visited_positions);
            }
        }
    }

    total
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<char>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}
