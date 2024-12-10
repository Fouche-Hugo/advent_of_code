use anyhow::Result;
use clap::Parser;
use std::{
    collections::HashSet,
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

fn trailhead_nines(
    map: &[Vec<usize>],
    start: Position,
    mut passed_positions: HashSet<Position>,
) -> HashSet<Position> {
    passed_positions.insert(start);
    let current = map[start.y][start.x];
    if current == 9 {
        return HashSet::from([start]);
    }

    let up = Position {
        x: start.x,
        y: (start.y as isize - 1) as usize,
    };
    let up_score = if map
        .get(up.y)
        .and_then(|line| line.get(up.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_nines(map, up, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let down = Position {
        x: start.x,
        y: start.y + 1,
    };
    let down_score = if map
        .get(down.y)
        .and_then(|line| line.get(down.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_nines(map, down, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let left = Position {
        x: (start.x as isize - 1) as usize,
        y: start.y,
    };
    let mut left_score = if map
        .get(left.y)
        .and_then(|line| line.get(left.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_nines(map, left, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let right = Position {
        x: start.x + 1,
        y: start.y,
    };
    let right_score = if map
        .get(right.y)
        .and_then(|line| line.get(right.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_nines(map, right, passed_positions)
    } else {
        HashSet::new()
    };

    left_score.extend(right_score);
    left_score.extend(up_score);
    left_score.extend(down_score);
    left_score
}

fn part1(map: Vec<Vec<usize>>) -> usize {
    let mut total = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                let scores = trailhead_nines(&map, Position { x, y }, HashSet::new());
                total += scores.len()
            }
        }
    }

    total
}

fn trailhead_rating(
    map: &[Vec<usize>],
    start: Position,
    mut passed_positions: HashSet<Position>,
) -> HashSet<Vec<Position>> {
    passed_positions.insert(start);
    let current = map[start.y][start.x];
    if current == 9 {
        passed_positions.insert(start);
        return HashSet::from([passed_positions.into_iter().collect()]);
    }

    let up = Position {
        x: start.x,
        y: (start.y as isize - 1) as usize,
    };
    let up_score = if map
        .get(up.y)
        .and_then(|line| line.get(up.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_rating(map, up, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let down = Position {
        x: start.x,
        y: start.y + 1,
    };
    let down_score = if map
        .get(down.y)
        .and_then(|line| line.get(down.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_rating(map, down, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let left = Position {
        x: (start.x as isize - 1) as usize,
        y: start.y,
    };
    let mut left_score = if map
        .get(left.y)
        .and_then(|line| line.get(left.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_rating(map, left, passed_positions.clone())
    } else {
        HashSet::new()
    };

    let right = Position {
        x: start.x + 1,
        y: start.y,
    };
    let right_score = if map
        .get(right.y)
        .and_then(|line| line.get(right.x))
        .is_some_and(|val| *val == current + 1)
    {
        trailhead_rating(map, right, passed_positions)
    } else {
        HashSet::new()
    };

    left_score.extend(right_score);
    left_score.extend(up_score);
    left_score.extend(down_score);
    left_score
}

fn part2(map: Vec<Vec<usize>>) -> usize {
    let mut total = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                let scores = trailhead_rating(&map, Position { x, y }, HashSet::new());
                total += scores.len()
            }
        }
    }

    total
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<usize>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect())
}
