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
    let disk = parse_input(&args.input_file)?;

    let result = if args.part2 { part2(disk) } else { part1(disk) };
    println!("{result}");
    Ok(())
}

fn part1(disk: Vec<DiskSpace>) -> usize {
    let (files, mut empty_spaces) = convert_disk(disk);

    let mut new_disk = vec![files[0]];
    let mut files: Vec<File> = files.into_iter().skip(1).rev().collect();
    let mut file_index = 0;
    let mut empty_index = 0;

    while file_index < files.len() {
        let file_size = files[file_index].size;
        let empty_space_size = empty_spaces[empty_index].size;

        if empty_space_size > file_size {
            new_disk.push(files[file_index]);
            empty_spaces[empty_index].size -= file_size;
            file_index += 1;
        } else if empty_space_size == file_size {
            new_disk.push(files[file_index]);
            empty_index += 1;
            file_index += 1;

            let files_len = files.len();
            new_disk.push(files[files_len - 1]);
            files = files.into_iter().take(files_len - 1).collect();
        } else {
            let file_part = File {
                id: files[file_index].id,
                size: empty_space_size,
            };
            new_disk.push(file_part);

            files[file_index].size -= empty_space_size;
            empty_index += 1;

            new_disk.push(files[files.len() - 1]);

            let files_len = files.len();
            files = files.into_iter().take(files_len - 1).collect();
        }
    }

    let mut total = 0;
    let mut current_index = 0;

    for file in new_disk {
        for _ in 0..file.size {
            total += current_index * file.id;
            current_index += 1;
        }
    }

    total
}

fn convert_disk(disk: Vec<DiskSpace>) -> (Vec<File>, Vec<Empty>) {
    let mut files = vec![];
    let mut empty = vec![];

    for disk_space in disk {
        match disk_space {
            DiskSpace::Empty { size } => empty.push(Empty { size }),
            DiskSpace::File { file_id, size } => files.push(File { id: file_id, size }),
        }
    }

    (files, empty)
}

#[derive(Debug, Clone, Copy)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Empty {
    size: usize,
}

#[derive(Debug)]
enum DiskSpace {
    File { file_id: usize, size: usize },
    Empty { size: usize },
}

impl From<File> for DiskSpace {
    fn from(value: File) -> Self {
        Self::File {
            file_id: value.id,
            size: value.size,
        }
    }
}

impl From<Empty> for DiskSpace {
    fn from(value: Empty) -> Self {
        Self::Empty { size: value.size }
    }
}

#[derive(Debug, Clone)]
struct File2 {
    start: usize,
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct Empty2 {
    start: usize,
    size: usize,
}

fn convert_disk_2(disk: Vec<DiskSpace>) -> (Vec<File2>, Vec<Empty2>) {
    let mut files = vec![];
    let mut empty = vec![];
    let mut current_index = 0;

    for disk_space in disk {
        match disk_space {
            DiskSpace::Empty { size } => {
                empty.push(Empty2 {
                    start: current_index,
                    size,
                });
                current_index += size;
            }
            DiskSpace::File { file_id, size } => {
                files.push(File2 {
                    start: current_index,
                    id: file_id,
                    size,
                });
                current_index += size
            }
        }
    }

    (files, empty)
}

fn part2(disk: Vec<DiskSpace>) -> usize {
    let (mut files, mut empty_spaces) = convert_disk_2(disk);

    let mut current_empty_space = 0;

    while current_empty_space < empty_spaces.len() {
        let empty_space = &mut empty_spaces[current_empty_space];

        let files_with_start_after: Vec<(usize, File2)> = files
            .iter()
            .enumerate()
            .filter(|(_, file)| file.start > empty_space.start)
            .map(|(index, file)| (index, file.clone()))
            .rev()
            .collect();
        let mut should_advance = true;
        for (file_index, file) in files_with_start_after {
            if file.size < empty_space.size {
                let moved_file = File2 {
                    id: file.id,
                    size: file.size,
                    start: empty_space.start,
                };
                files[file_index] = moved_file;
                empty_space.start += file.size;
                empty_space.size -= file.size;
            } else if file.size == empty_space.size {
                let moved_file = File2 {
                    id: file.id,
                    size: file.size,
                    start: empty_space.start,
                };
                files[file_index] = moved_file;
                empty_spaces.remove(current_empty_space);
                should_advance = false;
                break;
            }
        }

        files.sort_by_key(|file| file.start);
        if should_advance {
            current_empty_space += 1;
        }
    }

    let mut total = 0;
    for file in files {
        let mut current_index = file.start;
        for _ in 0..file.size {
            total += current_index * file.id;
            current_index += 1;
        }
    }

    total
}

fn parse_input(input_file: &Path) -> Result<Vec<DiskSpace>> {
    let file_input = std::fs::read_to_string(input_file)?;

    let mut disk = vec![];
    let mut file_id = 0;

    for i in (0..file_input.chars().count()).into_iter().step_by(2) {
        let file_space = DiskSpace::File {
            file_id,
            size: file_input.chars().nth(i).unwrap().to_string().parse()?,
        };

        disk.push(file_space);

        if let Some(empty_size) = file_input.chars().nth(i + 1) {
            let empty_space = DiskSpace::Empty {
                size: empty_size.to_string().parse()?,
            };
            disk.push(empty_space);
        }

        file_id += 1;
    }

    Ok(disk)
}
