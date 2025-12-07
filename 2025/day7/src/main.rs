use anyhow::Result;
use cached::proc_macro::cached;
use clap::Parser;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
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

    let result = if args.part2 {
        part2(Arc::new(map))
    } else {
        part1(map)
    };
    println!("{result}");
    Ok(())
}

fn part1(map: Vec<Vec<MapBlock>>) -> usize {
    let start_position_x = map[0]
        .iter()
        .position(|block| *block == MapBlock::Start)
        .unwrap();
    let start_position = Position {
        x: start_position_x,
        y: 0,
    };

    part1_explore(&map, start_position, &mut HashSet::new())
}

fn part1_explore(
    map: &[Vec<MapBlock>],
    start_position: Position,
    explored_positions: &mut HashSet<Position>,
) -> usize {
    for y in start_position.y..map.len() {
        let current_position = Position {
            x: start_position.x,
            y,
        };
        if explored_positions.contains(&current_position) {
            return 0;
        }
        explored_positions.insert(current_position);

        if map[current_position.y][current_position.x] == MapBlock::Split {
            let mut new_positions = vec![];
            if current_position.x > 0 {
                let left_position = Position {
                    x: current_position.x - 1,
                    y: current_position.y,
                };
                if !explored_positions.contains(&left_position) {
                    new_positions.push(left_position);
                }
            }
            if current_position.x < map[0].len() - 1 {
                let right_position = Position {
                    x: current_position.x + 1,
                    y: current_position.y,
                };
                if !explored_positions.contains(&right_position) {
                    new_positions.push(right_position);
                }
            }
            let splited_beam_total: usize = new_positions
                .into_iter()
                .map(|position| part1_explore(map, position, explored_positions))
                .sum();

            return 1 + splited_beam_total;
        }
    }

    0
}

fn part2(map: Arc<Vec<Vec<MapBlock>>>) -> usize {
    let start_position_x = map[0]
        .iter()
        .position(|block| *block == MapBlock::Start)
        .unwrap();
    let start_position = Position {
        x: start_position_x,
        y: 0,
    };

    part2_explore(map, start_position)
}

#[cached]
fn part2_explore(
    map: Arc<Vec<Vec<MapBlock>>>,
    start_position: Position,
    // explored_positions: &mut HashSet<Position>,
) -> usize {
    for y in start_position.y..map.len() {
        let current_position = Position {
            x: start_position.x,
            y,
        };

        if map[current_position.y][current_position.x] == MapBlock::Split {
            let mut new_positions = vec![];
            if current_position.x > 0 {
                let left_position = Position {
                    x: current_position.x - 1,
                    y: current_position.y,
                };
                new_positions.push(left_position);
            }
            if current_position.x < map[0].len() - 1 {
                let right_position = Position {
                    x: current_position.x + 1,
                    y: current_position.y,
                };
                new_positions.push(right_position);
            }
            let splited_beam_total: usize = new_positions
                .into_iter()
                .map(|position| part2_explore(map.clone(), position))
                .sum();

            return splited_beam_total;
        }
    }

    1
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum MapBlock {
    Dot,
    Split,
    Start,
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<MapBlock>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let map = file_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => MapBlock::Dot,
                    'S' => MapBlock::Start,
                    '^' => MapBlock::Split,
                    _ => panic!("failed to parse input file, got character: {c}"),
                })
                .collect()
        })
        .collect();

    Ok(map)
}
