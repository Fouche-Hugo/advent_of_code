use anyhow::Result;
use clap::Parser;
use image::{GrayImage, Luma};
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    input_file: PathBuf,
    #[clap(long)]
    part2: bool,
    #[arg(long)]
    width: usize,
    #[arg(long)]
    height: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let robots = parse_input(&args.input_file)?;

    let map = Map {
        width: args.width,
        height: args.height,
    };

    let result = if args.part2 {
        part2(robots, map)
    } else {
        part1(robots, map)
    };
    println!("{result}");
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
}

fn update_robot(robot: &mut Robot, map: &Map) {
    let original = robot.position;

    robot.position.x += robot.velocity.x;
    robot.position.y += robot.velocity.y;

    if robot.position.x >= map.width as isize {
        robot.position.x = robot.position.x - map.width as isize;
    } else if robot.position.x < 0 {
        robot.position.x = map.width as isize + robot.position.x;
    }

    if robot.position.y >= map.height as isize {
        robot.position.y = robot.position.y - map.height as isize;
    } else if robot.position.y < 0 {
        robot.position.y = map.height as isize + robot.position.y;
    }

    if robot.position.x >= map.width as isize {
        panic!("original: {original:?} ; {robot:?}");
    }

    assert!(robot.position.x >= 0);
    assert!(robot.position.x < map.width as isize);
    assert!(robot.position.y >= 0);
    assert!(robot.position.y < map.height as isize);
}

fn print_map(robots: &[Robot], map: &Map) {
    let mut tiles = vec![vec![0; map.width]; map.height];

    for robot in robots {
        tiles[robot.position.y as usize][robot.position.x as usize] += 1;
    }

    for (line_index, line) in tiles.iter().enumerate() {
        let line_value = line
            .iter()
            .enumerate()
            .map(|(index, val)| {
                if index == map.width / 2 {
                    format!("|{val}|")
                } else {
                    val.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        if line_index == map.height / 2 {
            println!("--------------------------------------------------------------");
        }
        println!("{}", line_value);
        if line_index == map.height / 2 {
            println!("--------------------------------------------------------------");
        }
    }
}

fn part1(mut robots: Vec<Robot>, map: Map) -> usize {
    for _ in 0..100 {
        for robot in &mut robots {
            update_robot(robot, &map);
        }
    }
    print_map(&robots, &map);

    let mut quadrants = (0, 0, 0, 0);

    for robot in robots {
        if robot.position.x < (map.width / 2) as isize
            && robot.position.y < (map.height / 2) as isize
        {
            quadrants.0 += 1;
        } else if robot.position.x < (map.width / 2) as isize
            && robot.position.y > (map.height / 2) as isize
        {
            quadrants.1 += 1;
        } else if robot.position.x > (map.width / 2) as isize
            && robot.position.y < (map.height / 2) as isize
        {
            quadrants.2 += 1;
        } else if robot.position.x > (map.width / 2) as isize
            && robot.position.y > (map.height / 2) as isize
        {
            quadrants.3 += 1
        }
    }

    println!("{quadrants:?}");

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn part2(mut robots: Vec<Robot>, map: Map) -> usize {
    for i in 0..10000 {
        for robot in &mut robots {
            update_robot(robot, &map);
        }
        save_map_as_image(&robots, &map, &format!("{i}.png"));
    }

    0
}

fn save_map_as_image(robots: &[Robot], map: &Map, image_name: &str) {
    let mut tiles = vec![vec![0; map.width]; map.height];

    for robot in robots {
        tiles[robot.position.y as usize][robot.position.x as usize] += 1;
    }

    let mut img = GrayImage::new(map.width as u32, map.height as u32);

    for (y, row) in tiles.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let pixel_value: u8 = if *value > 0 { 255 } else { 0 };

            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    img.save(&format!("2024d14/{image_name}")).unwrap();
}

fn parse_input(input_file: &Path) -> Result<Vec<Robot>> {
    let file_input = std::fs::read_to_string(input_file)?;

    Ok(file_input
        .lines()
        .map(|line| {
            let p_index = line.find("p").unwrap();
            let v_index = line.find("v").unwrap();
            let space_index = line.find(" ").unwrap();

            let p_str = &line[p_index + 2..space_index];
            let v_str = &line[v_index + 2..line.len()];

            let mut p_split = p_str.split(",");
            let mut v_split = v_str.split(",");

            let px = p_split.next().unwrap().parse().unwrap();
            let py = p_split.next().unwrap().parse().unwrap();

            let vx = v_split.next().unwrap().parse().unwrap();
            let vy = v_split.next().unwrap().parse().unwrap();

            Robot {
                position: Position { x: px, y: py },
                velocity: Velocity { x: vx, y: vy },
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::{update_robot, Map, Robot};

    #[test]
    fn test_robot() {
        let mut robot = Robot {
            position: crate::Position { x: 2, y: 4 },
            velocity: crate::Velocity { x: 2, y: -3 },
        };

        let map = Map {
            width: 11,
            height: 7,
        };

        update_robot(&mut robot, &map);
        assert_eq!(4, robot.position.x);
        assert_eq!(1, robot.position.y);

        update_robot(&mut robot, &map);
        assert_eq!(6, robot.position.x);
        assert_eq!(5, robot.position.y);

        for _ in 0..3 {
            update_robot(&mut robot, &map);
        }

        assert_eq!(1, robot.position.x);
        assert_eq!(3, robot.position.y);
    }
}
