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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Neighbour {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

fn compute_neighbour(map: &[Vec<char>], position: Position) -> Vec<Neighbour> {
    let plant = map[position.y][position.x];
    let mut neighbours = vec![];

    // NORTH
    if position.y > 0 && map[position.y - 1][position.x] == plant {
        neighbours.push(Neighbour::NORTH);
    }

    // SOUTH
    if position.y + 1 < map.len() && map[position.y + 1][position.x] == plant {
        neighbours.push(Neighbour::SOUTH);
    }

    // WEST
    if position.x > 0 && map[position.y][position.x - 1] == plant {
        neighbours.push(Neighbour::WEST);
    }

    // EAST
    if position.x + 1 < map.len() && map[position.y][position.x + 1] == plant {
        neighbours.push(Neighbour::EAST);
    }

    neighbours
}

fn compute_turns(map: &[Vec<char>], position: Position, neighbours: &[Neighbour]) -> usize {
    let value = if neighbours.len() == 0 {
        4
    } else if neighbours.len() == 1 {
        2
    } else if neighbours.len() == 2
        && (neighbours.contains(&Neighbour::WEST) && neighbours.contains(&Neighbour::EAST)
            || neighbours.contains(&Neighbour::NORTH) && neighbours.contains(&Neighbour::SOUTH))
    {
        0
    } else {
        let mut turns = 0;

        if neighbours.len() == 2 {
            turns += 1;
        }

        let (x, y) = (position.x, position.y);
        let plant = map[y][x];

        if neighbours.contains(&Neighbour::EAST)
            && neighbours.contains(&Neighbour::NORTH)
            && map[y - 1][x + 1] != plant
        {
            turns += 1;
        }

        if neighbours.contains(&Neighbour::NORTH)
            && neighbours.contains(&Neighbour::WEST)
            && map[y - 1][x - 1] != plant
        {
            turns += 1;
        }

        if neighbours.contains(&Neighbour::WEST)
            && neighbours.contains(&Neighbour::SOUTH)
            && map[y + 1][x - 1] != plant
        {
            turns += 1;
        }

        if neighbours.contains(&Neighbour::SOUTH)
            && neighbours.contains(&Neighbour::EAST)
            && map[y + 1][x + 1] != plant
        {
            turns += 1;
        }

        turns
    };

    value
}

fn visit_area_part2(
    map: &[Vec<char>],
    start: Position,
    mut visited_positions: HashSet<Position>,
) -> (usize, HashSet<Position>) {
    visited_positions.insert(start);

    let neighbours = compute_neighbour(map, start);
    let current_turns = compute_turns(map, start, &neighbours);

    let mut neighbour_turns = 0;

    for neighbour in neighbours {
        let neighbour_position = match neighbour {
            Neighbour::NORTH => Position {
                x: start.x,
                y: start.y - 1,
            },
            Neighbour::SOUTH => Position {
                x: start.x,
                y: start.y + 1,
            },
            Neighbour::EAST => Position {
                x: start.x + 1,
                y: start.y,
            },
            Neighbour::WEST => Position {
                x: start.x - 1,
                y: start.y,
            },
        };

        if !visited_positions.contains(&neighbour_position) {
            let (current_neighbour_turns, current_neighbour_visited_positions) =
                visit_area_part2(map, neighbour_position, visited_positions);
            visited_positions = current_neighbour_visited_positions;
            neighbour_turns += current_neighbour_turns;
        }
    }

    (neighbour_turns + current_turns, visited_positions)
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
