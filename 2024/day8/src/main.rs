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
    let equations = parse_input(&args.input_file)?;

    let result = if args.part2 {
        part2(equations)
    } else {
        part1(equations)
    };
    println!("{result}");
    Ok(())
}

#[derive(Debug)]
struct Antenna {
    location: Location,
    frequency: char,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

fn is_location_in_map(new_x: isize, new_y: isize, max_x: usize, max_y: usize) -> bool {
    new_x >= 0 && new_y >= 0 && new_x < max_x as isize && new_y < max_y as isize
}

fn part1(chars: Vec<Vec<char>>) -> usize {
    let max_x = chars.len();
    let max_y = chars[0].len();

    let antennas = get_antennas(chars);

    let mut locations_with_antinode = HashSet::new();

    for antenna1 in &antennas {
        for antenna2 in &antennas {
            if antenna1.frequency == antenna2.frequency && antenna1.location != antenna2.location {
                let x_diff = antenna1.location.x as isize - antenna2.location.x as isize;
                let y_diff = antenna1.location.y as isize - antenna2.location.y as isize;

                let location_x = antenna1.location.x as isize + x_diff;
                let location_y = antenna1.location.y as isize + y_diff;

                if is_location_in_map(location_x, location_y, max_x, max_y) {
                    locations_with_antinode.insert(Location {
                        x: location_x as usize,
                        y: location_y as usize,
                    });
                }
            }
        }
    }

    locations_with_antinode.len()
}

fn part2(chars: Vec<Vec<char>>) -> usize {
    let max_x = chars.len();
    let max_y = chars[0].len();

    let antennas = get_antennas(chars);

    let mut locations_with_antinode = HashSet::new();

    for antenna1 in &antennas {
        for antenna2 in &antennas {
            if antenna1.frequency == antenna2.frequency && antenna1.location != antenna2.location {
                let mut ratio = 0;

                let x_diff = antenna1.location.x as isize - antenna2.location.x as isize;
                let y_diff = antenna1.location.y as isize - antenna2.location.y as isize;

                let mut location_x = antenna1.location.x as isize;
                let mut location_y = antenna1.location.y as isize;

                while is_location_in_map(location_x, location_y, max_x, max_y) {
                    locations_with_antinode.insert(Location {
                        x: location_x as usize,
                        y: location_y as usize,
                    });

                    ratio += 1;

                    let x_diff = x_diff * ratio;
                    let y_diff = y_diff * ratio;

                    location_x = antenna1.location.x as isize + x_diff;
                    location_y = antenna1.location.y as isize + y_diff;
                }
            }
        }
    }

    locations_with_antinode.len()
}

fn get_antennas(chars: Vec<Vec<char>>) -> Vec<Antenna> {
    chars
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter(|(_, ch)| *ch != '.')
                .map(|(x, ch)| Antenna {
                    location: Location { x, y },
                    frequency: ch,
                })
                .collect::<Vec<Antenna>>()
        })
        .flatten()
        .collect()
}

fn parse_input(input_file: &Path) -> Result<Vec<Vec<char>>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| line.chars().collect())
        .collect())
}
