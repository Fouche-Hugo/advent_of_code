# Advent of Code

My personal solutions to the [Advent of Code](https://adventofcode.com) challenge.

## How to Use
- each day is a separate folder with python or rust files for each part of the challenge (for python, `solve.py` for part 1 and `solve2.py` for part 2)
- each python file is executable and will print the solution to the console
- each day has a `input.txt` file with the input data for that day

## How to Run
### Python solutions
- clone the repo
- `cd` into the directory `cd advent-of-code`
- run the python file for the day/part you want to see the solution for
    - example: `day1/solve.py`
- the solution will be printed to the console

### Rust solutions
- clone the repo
- `cd advent-of-code`
- run the following command (you can add a `--part2` at the end for part2)

    ```bash
    cargo run -p y{YEAR}d{DAY} -- {INPUT_FILE}
    ```