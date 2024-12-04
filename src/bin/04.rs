use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle = read_puzzle(reader)?;
        let mut total = 0_usize;

        for dir in Direction::iter() {
            total += get_lines(&puzzle, &dir)
                .into_iter()
                .map(|line| count_xmas(&line))
                .sum::<usize>();
        }

        Ok(total)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let puzzle = read_puzzle(reader)?;
        let mut total = 0_usize;
        let num_rows = puzzle.len();
        let num_cols = puzzle[0].len();
        let valid_neighbours = vec!["MMSS", "MSSM", "SMMS", "SSMM"];

        for row in 1..num_rows - 1 {
            for col in 1..num_cols - 1 {
                let center = puzzle[row][col];
                if center != 'A' {
                    continue;
                }

                let mut neighbors = String::new();
                neighbors.push(puzzle[row - 1][col - 1]);
                neighbors.push(puzzle[row - 1][col + 1]);
                neighbors.push(puzzle[row + 1][col + 1]);
                neighbors.push(puzzle[row + 1][col - 1]);

                if valid_neighbours.contains(&neighbors.as_str()) {
                    total += 1
                }
            }
        }

        Ok(total)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

type Puzzle = Vec<Vec<char>>;

#[derive(Debug, EnumIter)]
enum Direction {
    N,
    S,
    E,
    W,
    Ne,
    Nw,
    Se,
    Sw,
}

fn count_xmas(line: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    re.find_iter(line).count()
}

fn get_lines(puzzle: &Puzzle, dir: &Direction) -> Vec<String> {
    get_start_positions(puzzle, dir)
        .into_iter()
        .map(|(r, c)| get_line(puzzle, r, c, dir))
        .collect()
}

fn get_start_positions(puzzle: &Puzzle, dir: &Direction) -> Vec<(i32, i32)> {
    let num_rows = puzzle.len() as i32;
    let num_cols = puzzle[0].len() as i32;

    match dir {
        Direction::N => (0..num_cols)
            .into_iter()
            .map(|c| (num_rows - 1, c))
            .collect(),
        Direction::S => (0..num_cols).into_iter().map(|c| (0, c)).collect(),
        Direction::E => (0..num_rows).into_iter().map(|r| (r, 0)).collect(),
        Direction::W => (0..num_rows)
            .into_iter()
            .map(|r| (r, num_cols - 1))
            .collect(),
        Direction::Ne => {
            let mut ret: Vec<(i32, i32)> = (0..num_rows).into_iter().map(|r| (r, 0)).collect();
            ret.extend((1..num_cols).into_iter().map(|c| (num_rows - 1, c)));
            ret
        }
        Direction::Nw => {
            let mut ret: Vec<(i32, i32)> = (0..num_rows)
                .into_iter()
                .map(|r| (r, num_cols - 1))
                .collect();
            ret.extend((0..num_cols - 1).into_iter().map(|c| (num_rows - 1, c)));
            ret
        }
        Direction::Se => {
            let mut ret: Vec<(i32, i32)> = (0..num_rows).into_iter().map(|r| (r, 0)).collect();
            ret.extend((1..num_cols).into_iter().map(|c| (0, c)));
            ret
        }
        Direction::Sw => {
            let mut ret: Vec<(i32, i32)> = (0..num_rows)
                .into_iter()
                .map(|r| (r, num_cols - 1))
                .collect();
            ret.extend((0..num_cols - 1).into_iter().map(|c| (0, c)));
            ret
        }
    }
}

fn get_delta(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::N => (-1, 0),
        Direction::S => (1, 0),
        Direction::E => (0, 1),
        Direction::W => (0, -1),
        Direction::Ne => (-1, 1),
        Direction::Nw => (-1, -1),
        Direction::Se => (1, 1),
        Direction::Sw => (1, -1),
    }
}

fn get_line(puzzle: &Puzzle, start_row: i32, start_col: i32, dir: &Direction) -> String {
    let mut line = String::new();
    let mut row = start_row;
    let mut col = start_col;
    let num_rows = puzzle.len() as i32;
    let num_cols = puzzle[0].len() as i32;
    let delta = get_delta(dir);

    loop {
        if !is_valid_cell(row, col, num_rows, num_cols) {
            break;
        }
        line.push(puzzle[row as usize][col as usize]);
        row += delta.0;
        col += delta.1;
    }

    line
}

fn is_valid_cell(row: i32, col: i32, num_rows: i32, num_cols: i32) -> bool {
    (0..num_rows).contains(&row) && (0..num_cols).contains(&col)
}

fn read_puzzle(reader: impl BufRead) -> Result<Puzzle> {
    let mut ret = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        ret.push(row);
    }
    Ok(ret)
}
