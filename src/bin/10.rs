use adv_code_2024::*;
use adv_code_2024::grid::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = read_map(reader);
        let answer = map.count_scores();

        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = read_map(reader);
        let answer = map.count_ratings();

        Ok(answer)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}

#[derive(Debug, Clone)]
struct Map {
    grid: Grid<i32>,
}

impl Map {
    
    fn find_trail_ends(&self, start: &Position, height: i32) -> HashSet<Position> {
        if height == 9 {
            let mut ret = HashSet::new();
            ret.insert(start.clone());
            return ret;
        }

        let mut ret = HashSet::new();

        for direction in Direction::iter() {
            let next_pos = start.make_step(&direction);
            if !self.grid.is_valid_position(&next_pos) {
                continue;
            }
            let next_height = self.grid.cells[next_pos.row() as usize][next_pos.col() as usize];
            if next_height != height + 1 {
                continue;
            }
            let ends = self.find_trail_ends(&next_pos, next_height);
            for end in ends {
                ret.insert(end);
            }
        }

        ret
    }

    fn count_scores(&self) -> usize {
        let mut total = 0;

        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let height = self.grid.cells[row as usize][col as usize];
                if height == 0 {
                    let trail_ends = self.find_trail_ends(&Position::new(row, col), height);
                    total += trail_ends.len();
                }
            }
        }

        total
    }

    fn count_ratings(&self) -> usize {
        let mut total = 0;

        for row in 0..self.grid.num_rows {
            for col in 0..self.grid.num_cols {
                let height = self.grid.cells[row as usize][col as usize];
                if height == 0 {
                    total += self.find_trail_ratings(&Position::new(row, col), height);
                }
            }
        }

        total as usize
    }

    fn find_trail_ratings(&self, start: &Position, height: i32) -> i32 {
        if height == 9 {
            return 1;
        }

        let mut ret = 0;

        for direction in Direction::iter() {
            let next_pos = start.make_step(&direction);
            if !self.grid.is_valid_position(&next_pos) {
                continue;
            }
            let next_height = self.grid.cells[next_pos.row() as usize][next_pos.col() as usize];
            if next_height != height + 1 {
                continue;
            }
            ret += self.find_trail_ratings(&next_pos, next_height);
        }

        ret
    }
}

fn read_map<R: BufRead>(reader: R) -> Map {
    let lines = read_lines(reader);
    let num_rows = lines.len() as i32;
    let num_cols = lines[0].len() as i32;

    let mut heights = Vec::new();
    for line in lines.iter() {
        let mut row_heights = Vec::new();
        for ch in line.chars() {
            row_heights.push(ch.to_digit(10).unwrap() as i32);
        }
        heights.push(row_heights);
    }

    Map {
        grid: Grid{
            num_rows,
            num_cols,
            cells: heights,
        },
    }
}
